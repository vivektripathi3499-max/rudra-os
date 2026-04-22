use crate::println;
use core::mem::size_of;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ElfHeader {
    pub magic: [u8; 4],
    pub class: u8,
    pub endian: u8,
    pub version: u8,
    pub abi: u8,
    pub abi_version: u8,
    pub padding: [u8; 7],
    pub etype: u16,
    pub machine: u16,
    pub version2: u32,
    pub entry: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub flags: u32,
    pub offset: u64,
    pub vaddr: u64,
    pub paddr: u64,
    pub file_size: u64,
    pub mem_size: u64,
    pub align: u64,
}

pub fn load(data: &[u8]) -> Option<u64> {

    if data.len() < size_of::<ElfHeader>() {
        println!("ELF too small");
        return None;
    }

    let mut header: ElfHeader = unsafe { core::mem::zeroed() };

    unsafe {
        core::ptr::copy_nonoverlapping(
            data.as_ptr(),
            &mut header as *mut _ as *mut u8,
            size_of::<ElfHeader>(),
        );
    }

    if &header.magic != b"\x7FELF" {
        println!("Invalid ELF");
        return None;
    }

    println!("ELF detected");
    println!("Entry point: {:#x}", header.entry);

println!("ELF execution disabled (test mode)");

    // Program header parsing
    let ph_offset = 64;
    let ph_size = size_of::<ProgramHeader>();

    if data.len() >= ph_offset + ph_size {

        println!("Parsing program header...");

        let mut ph: ProgramHeader = unsafe { core::mem::zeroed() };

        unsafe {
            core::ptr::copy_nonoverlapping(
                data.as_ptr().add(ph_offset),
                &mut ph as *mut _ as *mut u8,
                ph_size,
            );
        }

        println!("Segment vaddr: {:#x}", ph.vaddr);
    }
Some(header.entry)
}
