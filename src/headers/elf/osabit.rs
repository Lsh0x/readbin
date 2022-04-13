pub struct OSABIT {}

/// Define the target operating system application binary interface
///
/// OSABIT define the possible values for the target operation system
/// Its store in the seventh byte of the identifaction 16 bits called `e_ident`
impl OSABIT {
    /// UNIX System V ABI
    pub const SYSV: u8 = 0;
    /// HP-UX
    pub const HPUX: u8 = 1;
    /// NetBSD.
    pub const NETBSD: u8 = 2;
    /// Object use GNU ELF extensions
    pub const GNU: u8 = 3;
    /// Sun Solaris
    pub const SOLARIS: u8 = 6;
    /// IBM AIX
    pub const AIX: u8 = 7;
    /// SGI Irix
    pub const IRIX: u8 = 8;
    /// FreeBSD
    pub const FREEBSD: u8 = 9;
    /// Compaq tru64 unix
    pub const TRU64: u8 = 10;
    /// Novell Modesto
    pub const MODESTO: u8 = 11;
    /// OpenBSD
    pub const OPENBSD: u8 = 12;
    /// ARM EABI
    pub const ARM_AEABI: u8 = 64;
    /// ARM
    pub const ARM: u8 = 97;
    /// Standalone embedded application
    pub const STANDALONE: u8 = 255;
}
