/// Identification offset for the elf binary `e_ident` field
pub struct Indent {}
#[warn(unused_must_use)]
impl Indent {
    //const MAG0: usize = 0;
    //const MAG1: usize = 1;
    //const MAG2: usize = 2;
    //const MAG3: usize = 3;
    pub const CLASS: usize = 4;
    pub const DATA: usize = 5;
    pub const VERSION: usize = 6;
    pub const OSABIT: usize = 7;
    pub const ABIVERSION: usize = 8;
    // const PAD: usize = 9;
}
