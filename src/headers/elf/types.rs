/// Define the object file type
pub struct TYPE {}

impl TYPE {
    /// No file type
    pub const NONE: u16 = 0;
    /// Relocatable file
    pub const REL: u16 = 1;
    /// Executable file
    pub const EXEC: u16 = 2;
    /// Share object file
    pub const DYN: u16 = 3;
    /// Core file
    pub const CORE: u16 = 4;
}
