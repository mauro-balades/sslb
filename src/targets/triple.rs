/// Represents a target triple, which consists of architecture, vendor, and target OS information.
pub struct TargetTriple {
    /// The architecture of the target.
    pub arch: Arch,
    /// The vendor of the target.
    pub vendor: Vendor,
    /// The target operating system.
    pub os: TargetOS,
}

/// Represents different architectures that a target can have.
pub enum Arch {
    /// x86 architecture.
    X86,
    /// x86_64 architecture.
    X86_64,
    /// ARM architecture.
    Arm,
    /// ARM 64-bit architecture (Aarch64).
    Aarch64,
    /// MIPS architecture.
    Mips,
    /// Little-endian MIPS architecture.
    Mipsel,
    /// PowerPC architecture.
    Powerpc,
    /// 64-bit PowerPC architecture.
    Powerpc64,
    /// IBM zSeries architecture (s390x).
    S390x,
}

/// Represents different vendors for the target.
pub enum Vendor {
    /// Unknown vendor.
    Unknown,
    /// Apple vendor.
    Apple,
    /// PC (Personal Computer) vendor.
    PC,
    /// Sony Computer Entertainment, Inc. (SCEI) vendor.
    SCEI,
}

/// Represents different target operating systems.
pub enum TargetOS {
    /// Unknown operating system.
    Unknown,
    /// Linux operating system.
    Linux,
    /// Apple's macOS (Darwin) operating system.
    Darwin,
    /// Microsoft Windows operating system.
    Windows,
    /// Android operating system.
    Android,
    /// Native Client (NaCl) operating system.
    NaCl,
    /// FreeBSD operating system.
    FreeBSD,
    /// OpenBSD operating system.
    OpenBSD,
    /// NetBSD operating system.
    NetBSD,
    /// DragonFlyBSD operating system.
    DragonFlyBSD,
    /// Oracle Solaris operating system.
    Solaris,
    /// Haiku operating system.
    Haiku,
    /// Minix operating system.
    Minix,
    /// Real-Time Executive for Multiprocessor Systems (RTEMS) operating system.
    RTEMS,
    /// Native Client (NaCl) ARM architecture operating system.
    NaClArm,
    /// Emscripten operating system (targeting web assembly).
    Emscripten,
    /// Google's Fuchsia operating system.
    Fuchsia,
}
