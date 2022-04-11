/// Define the architecture for the binary
///
/// class defined possible value for a binary
/// Its store in the fifth byte of the identifaction 16 bits called `e_ident`
pub struct Class {}
impl Class {
    /// Invalid class
    pub const NONE: u8 = 0;
    /// 32 bits object
    pub const ELF32: u8 = 1;
    /// 32 bits object
    pub const ELF64: u8 = 2;
}
