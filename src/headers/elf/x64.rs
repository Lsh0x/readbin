use crate::utils::cow_struct;
use std::fmt;
use std::mem::size_of;
use std::borrow::Cow;

use super::class::Class;
use super::data::DATA;
use super::identification::Indent;
use super::osabit::OSABIT;
use super::types::TYPE;
use super::version::VERSION;

/// Format of Executable and Linking Format (ELF64) files
///
/// The header file <elf.h> defines the format of ELF executable
/// binary files.  Amongst these files are normal executable files,
/// relocatable object files, core files, and shared objects.
///
/// An executable file using the ELF file format consists of an ELF
/// header, followed by a program header table or a section header
/// table, or both.  The ELF header is always at offset zero of the
/// file.  The program header table and the section header table's
/// offset in the file are defined in the ELF header.  The two tables
/// describe the rest of the particularities of the file.
/// Sources:
/// * https://www.man7.org/linux/man-pages/man5/elf.5.html
/// * https://uclibc.org/docs/elf-64-gen.pdf
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct x64 {
    /// ELF identifaction
    pub e_ident: [u8; 16],
    /// object file type
    pub e_type: u16,
    /// machine type
    pub e_machine: u16,
    /// object file version
    pub e_version: u32,
    /// Entry point address
    pub e_entry: u64,
    /// program header offset
    pub e_phoff: u64,
    /// section header offset
    pub e_shoff: u64,
    /// processor specific flags
    pub e_flags: u32,
    /// elf header size
    pub e_ehsize: u16,
    /// Size of program header entry
    pub e_phentsize: u16,
    /// numbers of program header entries
    pub e_phnum: u16,
    /// size of section header entry
    pub e_shentsize: u16,
    /// number of section header entries
    pub e_shnum: u16,
    /// section name string table index
    pub e_shstrndx: u16,
}

impl x64 {
    pub const SIZE: usize = size_of::<Self>();
}

pub fn from_bytes(data: &[u8]) -> Option<Cow<x64>> {
    if data.len() < x64::SIZE {
        return None;
    }
    let (header_bytes, _data) = data.split_at(x64::SIZE);
    cow_struct::<x64>(header_bytes)
}

impl fmt::Display for x64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ELF Hearder:").unwrap();

        // Write indent
        writeln!(
            f,
            "  Magic:  {}",
            self.e_ident
                .iter()
                .map(|hex| format!("{:02X?} ", hex))
                .collect::<String>()
        )
        .unwrap();

        // Write class
        let class = match self.e_ident[Indent::CLASS] {
            Class::NONE => "Invalid class",
            Class::ELF32 => "ELF32",
            Class::ELF64 => "ELF64",
            _ => "Warning: unknown class",
        };
        writeln!(f, "  Class:\t\t\t\t{},", class).unwrap();

        // write data encoding
        let data_encoding = match self.e_ident[Indent::DATA] {
            DATA::NONE => "Unknown data encoding",
            DATA::BE => "2's complement, big endian",
            DATA::LE => "2's complement, little endian",
            _ => "Warning: unknow data encoding",
        };
        writeln!(f, "  Data:\t\t\t\t\t{}", data_encoding).unwrap();

        // write current number version of elf specification
        let current = format!("{} (current)", self.e_ident[Indent::VERSION]);
        let version = match self.e_ident[Indent::VERSION] {
            VERSION::NONE => "Invalid version",
            VERSION::CURRENT => current.as_str(),
            _ => "Warning: unknow version",
        };
        writeln!(f, "  Version:\t\t\t\t{}", version).unwrap();

        // write target os application binary interface
        let osabit = match self.e_ident[Indent::OSABIT] {
            OSABIT::SYSV => "UNIX System V ABI",
            OSABIT::HPUX => "HP-UX",
            OSABIT::NETBSD => "NetBSD",
            OSABIT::GNU => "Object use GNU ELF extensions",
            OSABIT::SOLARIS => "Sun Solaris",
            OSABIT::AIX => "IBM AIX",
            OSABIT::IRIX => "SGI Irix",
            OSABIT::FREEBSD => "FreeBSD",
            OSABIT::TRU64 => "Compaq tru64 unix",
            OSABIT::MODESTO => "Novell Modesto",
            OSABIT::OPENBSD => "OpenBSD",
            OSABIT::ARM_AEABI => "ARM AEABI",
            OSABIT::ARM => "ARM",
            OSABIT::STANDALONE => "Standalone embedded application",
            _ => "Warning: unknow operating system target",
        };
        writeln!(f, "  OS/ABI:\t\t\t\t{}", osabit).unwrap();

        let abi_version_message = match self.e_ident[Indent::ABIVERSION] {
            0 => "0",
            _ => "Warning: Not compatible with the specification",
        };
        writeln!(f, "  ABI Version:\t\t\t\t{}", abi_version_message).unwrap();

        // write object file type
        let obj_type = match self.e_type {
            TYPE::NONE => "NONE (No file type)",
            TYPE::REL => "REL (Relocatable file)",
            TYPE::EXEC => "EXEC (Executable file)",
            TYPE::DYN => "DYN (Share object file)",
            TYPE::CORE => "CORE (Core file)",
            _ => "Warning: unknow object file type",
        };
        writeln!(f, "  Type: \t\t\t\t{}", obj_type).unwrap();

        // TODO add machine

        // write current number version of elf specification
        return writeln!(f, "  Version:\t\t\t\t{:#x}", self.e_version);
    }
}
