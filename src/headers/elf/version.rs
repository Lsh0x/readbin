/// Define the version number of the elf specification
///
/// Version define possible values for the version number
/// Its store in the seventh byte of the identifaction 16 bits called `e_ident`
pub struct VERSION {}
impl VERSION {
    /// invalid version
    pub const NONE: u8 = 0;
    /// current elf version
    pub const CURRENT: u8 = 1;
}
