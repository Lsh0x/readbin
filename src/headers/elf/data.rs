/// Define the data encoding for the processor-specific data
///
/// Data define possible values for the encoding
/// Its store in the sixth byte of the identifaction 16 bits called `e_ident`
pub struct DATA {}
impl DATA {
    /// Unknow data encoding
    pub const NONE: u8 = 0;
    /// Little endian data encoding
    pub const LE: u8 = 1;
    /// Big endian data encoding
    pub const BE: u8 = 2;
}
