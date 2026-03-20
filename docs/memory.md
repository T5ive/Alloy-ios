# Memory & Hooking

Alloy's memory subsystem (powered by [`specter-mem`](https://crates.io/crates/specter-mem)) provides hooking, patching, scanning, and shellcode execution for ARM64 iOS targets.

## Inline Hooking

Replace a function at a given address with your own implementation. The original function is preserved via a trampoline.

```rust
#[no_mangle]
extern "C" fn my_replacement() {
    logger::info("Hook hit!");
}

let target = 0x100000; // RVA of the target function
let replacement = my_replacement as usize;

unsafe {
    if let Ok(hook) = specter::memory::manipulation::hook::install(target, replacement) {
        logger::info(&format!("Trampoline at {:#x}", hook.trampoline() as usize));
    }
}
```

## Hardware Breakpoint Hooking

Uses ARM64 debug registers for stealthier hooks that don't modify code in memory.

```rust
#[no_mangle]
extern "C" fn my_brk_replacement() {
    logger::info("Breakpoint hook hit!");
}

let target = 0x100004;
let replacement = my_brk_replacement as usize;

unsafe {
    if let Ok(_brk) = specter::memory::platform::breakpoint::install(target, replacement) {
        logger::info("Hardware breakpoint hook installed!");
    }
}
```

## Memory Patching

### Hex Patch

Write raw hex bytes at a given RVA. Uses stealth `mach_vm_remap` when possible.

```rust
let target_rva = 0x100008;
let patch_hex = "C0 03 5F D6"; // ARM64 RET instruction

if let Ok(_patch) = specter::memory::manipulation::patch::apply(target_rva, patch_hex) {
    logger::info("Hex patch applied!");
}
```

### Assembly Patch

Write patches using the `jit-assembler` crate for type-safe ARM64 assembly:

```rust
use jit_assembler::aarch64::{builder::Aarch64InstructionBuilder, Register};

let target_rva = 0x10000C;

if let Ok(_patch) = specter::memory::manipulation::patch::apply_asm(
    target_rva,
    |b: &mut Aarch64InstructionBuilder| {
        b.movz(Register(0), 1, 0) // MOV X0, #1
         .ret()                    // RET
    },
) {
    logger::info("ASM patch applied!");
}
```

## Code Caves

Find and allocate unused memory regions for storing custom code.

```rust
if let Ok(cave) = specter::memory::info::code_cave::allocate_cave(32) {
    logger::info(&format!("Cave at {:#x}", cave.address));

    // Free when done
    let _ = specter::memory::info::code_cave::free_cave(cave.address);
}
```

## Shellcode Execution

Load and execute custom ARM64 shellcode in an executable memory region.

```rust
let shellcode: &[u32] = &[
    0xD2800540, // MOV X0, #42
    0xD65F03C0, // RET
];

match specter::memory::allocation::shellcode::ShellcodeBuilder::from_instructions(shellcode).load() {
    Ok(loaded) => unsafe {
        let result: usize = loaded.execute();
        logger::info(&format!("Result: {}", result));
    },
    Err(err) => {
        logger::error(&format!("Shellcode failed: {:?}", err));
    }
}
```

## Address Types

- **RVA** (Relative Virtual Address): Offset from the base of the target image. Most Alloy APIs accept RVAs and resolve them internally using the configured target image.
- **Absolute Address**: Full virtual memory address. Used internally after resolution.

The target image is configured via `config::set_target_image_name()` or auto-detected (defaults to `UnityFramework` if loaded, otherwise the main executable).
