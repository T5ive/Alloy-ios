//! Dynamic library image lookup utilities

use crate::utils::logger;
use std::ffi::CStr;

use mach2::dyld::{
    _dyld_get_image_header, _dyld_get_image_name, _dyld_get_image_vmaddr_slide, _dyld_image_count,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("Image not found: {0}")]
    NotFound(String),
}

pub fn get_image_base(image_name: &str) -> Result<usize, ImageError> {
    unsafe {
        let count = _dyld_image_count();

        for i in 0..count {
            let name_ptr = _dyld_get_image_name(i);
            if name_ptr.is_null() {
                continue;
            }

            let name = CStr::from_ptr(name_ptr).to_string_lossy();
            if name.contains(image_name) {
                let header = _dyld_get_image_header(i);
                let slide = _dyld_get_image_vmaddr_slide(i);

                logger::info(&format!(
                    "Found image: {} (Index: {}, Base: {:p}, Slide: {:#x})",
                    name, i, header, slide
                ));

                return Ok(header as usize);
            }
        }
    }

    logger::warning(&format!("Image not found: {}", image_name));
    Err(ImageError::NotFound(image_name.to_string()))
}
