//! Memory scanning and pattern matching utilities

use crate::memory::image;
use crate::utils::logger;
use mach2::{
    kern_return::KERN_SUCCESS,
    traps::mach_task_self,
    vm::mach_vm_region,
    vm_prot::VM_PROT_READ,
    vm_region::{vm_region_basic_info_64, VM_REGION_BASIC_INFO_64},
    vm_types::{mach_vm_address_t, mach_vm_size_t},
};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Invalid pattern format: {0}")]
    InvalidPattern(String),
    #[error("Pattern not found")]
    NotFound,
    #[error("Memory access violation at {0:#x}")]
    MemoryAccessViolation(usize),
    #[error("Invalid memory region")]
    InvalidRegion,
    #[error("Image not found: {0}")]
    ImageNotFound(#[from] super::image::ImageError),
}

static SCAN_CACHE: Lazy<Mutex<HashMap<String, Vec<usize>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn parse_ida_pattern(pattern: &str) -> Result<(Vec<u8>, String), ScanError> {
    let parts: Vec<&str> = pattern.split_whitespace().collect();
    let mut bytes = Vec::new();
    let mut mask = String::new();
    for part in parts {
        if part == "??" {
            bytes.push(0);
            mask.push('?');
        } else if part.len() == 2 {
            bytes.push(
                u8::from_str_radix(part, 16)
                    .map_err(|_| ScanError::InvalidPattern(format!("Invalid hex: {}", part)))?,
            );
            mask.push('x');
        } else {
            return Err(ScanError::InvalidPattern(format!(
                "Invalid pattern part: {}",
                part
            )));
        }
    }
    if bytes.is_empty() {
        return Err(ScanError::InvalidPattern("Empty pattern".to_string()));
    }
    Ok((bytes, mask))
}

pub fn scan_pattern(
    start: usize,
    size: usize,
    pattern: &[u8],
    mask: &str,
) -> Result<Vec<usize>, ScanError> {
    if pattern.is_empty() || pattern.len() != mask.len() {
        return Err(ScanError::InvalidPattern(
            "Pattern and mask length mismatch".to_string(),
        ));
    }
    if !is_readable_memory(start, size) {
        return Err(ScanError::MemoryAccessViolation(start));
    }
    let mut results = Vec::new();
    let end = start + size - pattern.len();
    for addr in start..=end {
        if pattern_match(addr, pattern, mask) {
            results.push(addr);
        }
    }
    if results.is_empty() {
        return Err(ScanError::NotFound);
    }
    Ok(results)
}

pub fn scan_pattern_first(
    start: usize,
    size: usize,
    pattern: &[u8],
    mask: &str,
) -> Result<usize, ScanError> {
    if pattern.is_empty() || pattern.len() != mask.len() {
        return Err(ScanError::InvalidPattern(
            "Pattern and mask length mismatch".to_string(),
        ));
    }
    if !is_readable_memory(start, size) {
        return Err(ScanError::MemoryAccessViolation(start));
    }
    let end = start + size - pattern.len();
    for addr in start..=end {
        if pattern_match(addr, pattern, mask) {
            return Ok(addr);
        }
    }
    Err(ScanError::NotFound)
}

pub fn scan_ida_pattern(
    start: usize,
    size: usize,
    ida_pattern: &str,
) -> Result<Vec<usize>, ScanError> {
    let (bytes, mask) = parse_ida_pattern(ida_pattern)?;
    scan_pattern(start, size, &bytes, &mask)
}

pub fn scan_ida_pattern_first(
    start: usize,
    size: usize,
    ida_pattern: &str,
) -> Result<usize, ScanError> {
    let (bytes, mask) = parse_ida_pattern(ida_pattern)?;
    scan_pattern_first(start, size, &bytes, &mask)
}

pub fn scan_image(image_name: &str, ida_pattern: &str) -> Result<Vec<usize>, ScanError> {
    let base = image::get_image_base(image_name)?;
    let sections = get_image_sections(base)?;
    let (bytes, mask) = parse_ida_pattern(ida_pattern)?;
    let mut all_results = Vec::new();
    for (section_start, section_size) in sections {
        if let Ok(mut results) = scan_pattern(section_start, section_size, &bytes, &mask) {
            all_results.append(&mut results);
        }
    }
    if all_results.is_empty() {
        return Err(ScanError::NotFound);
    }
    Ok(all_results)
}

pub fn scan_image_first(image_name: &str, ida_pattern: &str) -> Result<usize, ScanError> {
    let base = image::get_image_base(image_name)?;
    let sections = get_image_sections(base)?;
    let (bytes, mask) = parse_ida_pattern(ida_pattern)?;
    for (section_start, section_size) in sections {
        if let Ok(addr) = scan_pattern_first(section_start, section_size, &bytes, &mask) {
            return Ok(addr);
        }
    }
    Err(ScanError::NotFound)
}

pub fn aob_scan(
    start: usize,
    size: usize,
    pattern: &[u8],
    mask: &str,
) -> Result<Vec<usize>, ScanError> {
    scan_pattern(start, size, pattern, mask)
}

pub fn aob_scan_first(
    start: usize,
    size: usize,
    pattern: &[u8],
    mask: &str,
) -> Result<usize, ScanError> {
    scan_pattern_first(start, size, pattern, mask)
}

pub fn scan_pattern_cached(
    start: usize,
    size: usize,
    ida_pattern: &str,
) -> Result<Vec<usize>, ScanError> {
    let cache_key = format!("{:#x}_{:#x}_{}", start, size, ida_pattern);
    {
        let cache = SCAN_CACHE.lock();
        if let Some(cached) = cache.get(&cache_key) {
            logger::info(&format!("Cache hit for pattern: {}", ida_pattern));
            return Ok(cached.clone());
        }
    }
    let results = scan_ida_pattern(start, size, ida_pattern)?;
    {
        SCAN_CACHE.lock().insert(cache_key, results.clone());
    }
    Ok(results)
}

pub fn clear_cache() {
    SCAN_CACHE.lock().clear();
    logger::info("Scan cache cleared");
}

pub fn scan_asm<F>(start: usize, size: usize, build: F) -> Result<Vec<usize>, ScanError>
where
    F: FnOnce(
        &mut jit_assembler::aarch64::Aarch64InstructionBuilder,
    ) -> &mut jit_assembler::aarch64::Aarch64InstructionBuilder,
{
    use jit_assembler::{aarch64::Aarch64InstructionBuilder, common::InstructionBuilder};
    let mut builder = Aarch64InstructionBuilder::new();
    build(&mut builder);
    let instructions = builder.instructions();
    if instructions.is_empty() {
        return Err(ScanError::InvalidPattern("Empty ASM pattern".into()));
    }
    let bytes: Vec<u8> = instructions
        .iter()
        .flat_map(|instr| instr.0.to_le_bytes())
        .collect();
    let mask = "x".repeat(bytes.len());
    scan_pattern(start, size, &bytes, &mask)
}

pub fn scan_asm_first<F>(start: usize, size: usize, build: F) -> Result<usize, ScanError>
where
    F: FnOnce(
        &mut jit_assembler::aarch64::Aarch64InstructionBuilder,
    ) -> &mut jit_assembler::aarch64::Aarch64InstructionBuilder,
{
    use jit_assembler::{aarch64::Aarch64InstructionBuilder, common::InstructionBuilder};
    let mut builder = Aarch64InstructionBuilder::new();
    build(&mut builder);
    let instructions = builder.instructions();
    if instructions.is_empty() {
        return Err(ScanError::InvalidPattern("Empty ASM pattern".into()));
    }
    let bytes: Vec<u8> = instructions
        .iter()
        .flat_map(|instr| instr.0.to_le_bytes())
        .collect();
    let mask = "x".repeat(bytes.len());
    scan_pattern_first(start, size, &bytes, &mask)
}

pub fn scan_image_asm<F>(image_name: &str, build: F) -> Result<Vec<usize>, ScanError>
where
    F: FnOnce(
        &mut jit_assembler::aarch64::Aarch64InstructionBuilder,
    ) -> &mut jit_assembler::aarch64::Aarch64InstructionBuilder,
{
    use jit_assembler::{aarch64::Aarch64InstructionBuilder, common::InstructionBuilder};
    let mut builder = Aarch64InstructionBuilder::new();
    build(&mut builder);
    let instructions = builder.instructions();
    if instructions.is_empty() {
        return Err(ScanError::InvalidPattern("Empty ASM pattern".into()));
    }
    let bytes: Vec<u8> = instructions
        .iter()
        .flat_map(|instr| instr.0.to_le_bytes())
        .collect();
    let mask = "x".repeat(bytes.len());
    let base = image::get_image_base(image_name)?;
    let sections = get_image_sections(base)?;
    let mut all_results = Vec::new();
    for (section_start, section_size) in sections {
        if let Ok(mut results) = scan_pattern(section_start, section_size, &bytes, &mask) {
            all_results.append(&mut results);
        }
    }
    if all_results.is_empty() {
        return Err(ScanError::NotFound);
    }
    Ok(all_results)
}

pub fn scan_image_asm_first<F>(image_name: &str, build: F) -> Result<usize, ScanError>
where
    F: FnOnce(
        &mut jit_assembler::aarch64::Aarch64InstructionBuilder,
    ) -> &mut jit_assembler::aarch64::Aarch64InstructionBuilder,
{
    use jit_assembler::{aarch64::Aarch64InstructionBuilder, common::InstructionBuilder};
    let mut builder = Aarch64InstructionBuilder::new();
    build(&mut builder);
    let instructions = builder.instructions();
    if instructions.is_empty() {
        return Err(ScanError::InvalidPattern("Empty ASM pattern".into()));
    }
    let bytes: Vec<u8> = instructions
        .iter()
        .flat_map(|instr| instr.0.to_le_bytes())
        .collect();
    let mask = "x".repeat(bytes.len());
    let base = image::get_image_base(image_name)?;
    let sections = get_image_sections(base)?;
    for (section_start, section_size) in sections {
        if let Ok(addr) = scan_pattern_first(section_start, section_size, &bytes, &mask) {
            return Ok(addr);
        }
    }
    Err(ScanError::NotFound)
}

fn is_readable_memory(addr: usize, size: usize) -> bool {
    unsafe {
        let task = mach_task_self();
        let mut address = addr as mach_vm_address_t;
        let mut region_size: mach_vm_size_t = 0;
        let mut info = vm_region_basic_info_64::default();
        let mut info_count = VM_REGION_BASIC_INFO_64;
        let mut object_name = 0;
        let kr = mach_vm_region(
            task,
            &mut address,
            &mut region_size,
            VM_REGION_BASIC_INFO_64,
            &mut info as *mut _ as *mut i32,
            &mut info_count as *mut _ as *mut u32,
            &mut object_name,
        );
        if kr != KERN_SUCCESS {
            return false;
        }
        if address > addr as mach_vm_address_t {
            return false;
        }
        let region_end = address + region_size;
        let requested_end = (addr + size) as mach_vm_address_t;
        if requested_end > region_end {
            return false;
        }
        (info.protection & VM_PROT_READ) != 0
    }
}

fn get_image_sections(base: usize) -> Result<Vec<(usize, usize)>, ScanError> {
    let mut sections = Vec::new();
    unsafe {
        let task = mach_task_self();
        let mut address = base as mach_vm_address_t;
        let end_address = address + 0x10000000;
        while address < end_address {
            let mut region_size: mach_vm_size_t = 0;
            let mut info = vm_region_basic_info_64::default();
            let mut info_count = VM_REGION_BASIC_INFO_64;
            let mut object_name = 0;
            let kr = mach_vm_region(
                task,
                &mut address,
                &mut region_size,
                VM_REGION_BASIC_INFO_64,
                &mut info as *mut _ as *mut i32,
                &mut info_count as *mut _ as *mut u32,
                &mut object_name,
            );
            if kr != KERN_SUCCESS {
                break;
            }
            if (info.protection & VM_PROT_READ) != 0 {
                sections.push((address as usize, region_size as usize));
            }
            address += region_size;
        }
    }
    if sections.is_empty() {
        return Err(ScanError::InvalidRegion);
    }
    Ok(sections)
}

#[inline]
fn pattern_match(addr: usize, pattern: &[u8], mask: &str) -> bool {
    unsafe {
        let ptr = addr as *const u8;
        for (i, &byte) in pattern.iter().enumerate() {
            if mask.as_bytes()[i] == b'x' && *ptr.add(i) != byte {
                return false;
            }
        }
        true
    }
}
