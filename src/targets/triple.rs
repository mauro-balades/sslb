use crate::error::Error;
use std::fmt::Debug;
use std::fmt::Formatter;

/// Represents a target triple, which consists of architecture, vendor, and target OS information.
#[derive(PartialEq, Clone)]
pub struct TargetTriple {
    /// The architecture of the target.
    pub arch: Arch,
    /// The vendor of the target.
    pub vendor: Vendor,
    /// The target operating system.
    pub os: TargetOS,
}

/// Represents different architectures that a target can have.
#[derive(PartialEq, Clone, Debug, Copy)]
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
#[derive(PartialEq, Clone, Debug, Copy)]
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
#[derive(PartialEq, Clone, Debug, Copy)]
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

impl TargetTriple {
    /// Creates a new target triple from the given string.
    pub fn new(triple: &str) -> Result<Self, Error> {
        let mut parts = triple.split('-');
        let arch = parts.next().ok_or(Error::InvalidTargetTriple)?;
        let vendor = parts.next().ok_or(Error::InvalidTargetTriple)?;
        let os = parts.next().ok_or(Error::InvalidTargetTriple)?;

        let arch = match arch {
            "i686" | "i586" | "i486" | "i386" => Arch::X86,
            "x86_64" => Arch::X86_64,
            "arm" => Arch::Arm,
            "aarch64" => Arch::Aarch64,
            "mips" => Arch::Mips,
            "mipsel" => Arch::Mipsel,
            "powerpc" => Arch::Powerpc,
            "powerpc64" => Arch::Powerpc64,
            "s390x" => Arch::S390x,
            _ => return Err(Error::InvalidTargetTriple),
        };

        let vendor = match vendor {
            "unknown" => Vendor::Unknown,
            "apple" => Vendor::Apple,
            "pc" => Vendor::PC,
            "scei" => Vendor::SCEI,
            _ => return Err(Error::InvalidTargetTriple),
        };

        let os = match os {
            "unknown" => TargetOS::Unknown,
            "linux" => TargetOS::Linux,
            "darwin" => TargetOS::Darwin,
            "windows" => TargetOS::Windows,
            "android" => TargetOS::Android,
            "nacl" => TargetOS::NaCl,
            "freebsd" => TargetOS::FreeBSD,
            "openbsd" => TargetOS::OpenBSD,
            "netbsd" => TargetOS::NetBSD,
            "dragonfly" => TargetOS::DragonFlyBSD,
            "solaris" => TargetOS::Solaris,
            "haiku" => TargetOS::Haiku,
            "minix" => TargetOS::Minix,
            "rtems" => TargetOS::RTEMS,
            "naclarm" => TargetOS::NaClArm,
            "emscripten" => TargetOS::Emscripten,
            "fuchsia" => TargetOS::Fuchsia,
            _ => return Err(Error::InvalidTargetTriple),
        };

        Ok(TargetTriple { arch, vendor, os })
    }

    pub fn from_parts(arch: Arch, vendor: Vendor, os: TargetOS) -> Self {
        TargetTriple { arch, vendor, os }
    }

    /// Returns the target triple as a string.
    pub fn as_str(&self) -> String {
        let arch = match self.arch {
            Arch::X86 => "i686",
            Arch::X86_64 => "x86_64",
            Arch::Arm => "arm",
            Arch::Aarch64 => "aarch64",
            Arch::Mips => "mips",
            Arch::Mipsel => "mipsel",
            Arch::Powerpc => "powerpc",
            Arch::Powerpc64 => "powerpc64",
            Arch::S390x => "s390x",
        };

        let vendor = match self.vendor {
            Vendor::Unknown => "unknown",
            Vendor::Apple => "apple",
            Vendor::PC => "pc",
            Vendor::SCEI => "scei",
        };

        let os = match self.os {
            TargetOS::Unknown => "unknown",
            TargetOS::Linux => "linux",
            TargetOS::Darwin => "darwin",
            TargetOS::Windows => "windows",
            TargetOS::Android => "android",
            TargetOS::NaCl => "nacl",
            TargetOS::FreeBSD => "freebsd",
            TargetOS::OpenBSD => "openbsd",
            TargetOS::NetBSD => "netbsd",
            TargetOS::DragonFlyBSD => "dragonfly",
            TargetOS::Solaris => "solaris",
            TargetOS::Haiku => "haiku",
            TargetOS::Minix => "minix",
            TargetOS::RTEMS => "rtems",
            TargetOS::NaClArm => "naclarm",
            TargetOS::Emscripten => "emscripten",
            TargetOS::Fuchsia => "fuchsia",
        };

        format!("{}-{}-{}", arch, vendor, os)
    }

    /// It creates a new target triple from the current host.
    pub fn from_host() -> Result<Self, Error> {
        let arch = if cfg!(target_arch = "x86") {
            Arch::X86
        } else if cfg!(target_arch = "x86_64") {
            Arch::X86_64
        } else if cfg!(target_arch = "arm") {
            Arch::Arm
        } else if cfg!(target_arch = "aarch64") {
            Arch::Aarch64
        } else if cfg!(target_arch = "mips") {
            Arch::Mips
        } else if cfg!(target_arch = "mipsel") {
            Arch::Mipsel
        } else if cfg!(target_arch = "powerpc") {
            Arch::Powerpc
        } else if cfg!(target_arch = "powerpc64") {
            Arch::Powerpc64
        } else if cfg!(target_arch = "s390x") {
            Arch::S390x
        } else {
            return Err(Error::InvalidTargetTriple);
        };

        let vendor = if cfg!(target_vendor = "unknown") {
            Vendor::Unknown
        } else if cfg!(target_vendor = "apple") {
            Vendor::Apple
        } else if cfg!(target_vendor = "pc") {
            Vendor::PC
        } else if cfg!(target_vendor = "scei") {
            Vendor::SCEI
        } else {
            return Err(Error::InvalidTargetTriple);
        };

        let os = if cfg!(target_os = "unknown") {
            TargetOS::Unknown
        } else if cfg!(target_os = "linux") {
            TargetOS::Linux
        } else if cfg!(target_os = "darwin") {
            TargetOS::Darwin
        } else if cfg!(target_os = "windows") {
            TargetOS::Windows
        } else if cfg!(target_os = "android") {
            TargetOS::Android
        } else if cfg!(target_os = "nacl") {
            TargetOS::NaCl
        } else if cfg!(target_os = "freebsd") {
            TargetOS::FreeBSD
        } else if cfg!(target_os = "openbsd") {
            TargetOS::OpenBSD
        } else if cfg!(target_os = "netbsd") {
            TargetOS::NetBSD
        } else if cfg!(target_os = "dragonfly") {
            TargetOS::DragonFlyBSD
        } else if cfg!(target_os = "solaris") {
            TargetOS::Solaris
        } else if cfg!(target_os = "haiku") {
            TargetOS::Haiku
        } else if cfg!(target_os = "minix") {
            TargetOS::Minix
        } else if cfg!(target_os = "rtems") {
            TargetOS::RTEMS
        } else if cfg!(target_os = "naclarm") {
            TargetOS::NaClArm
        } else if cfg!(target_os = "emscripten") {
            TargetOS::Emscripten
        } else if cfg!(target_os = "fuchsia") {
            TargetOS::Fuchsia
        } else {
            return Err(Error::InvalidTargetTriple);
        };

        Ok(TargetTriple { arch, vendor, os })
    }

    /// Returns the target triple for the current host.
    pub fn host() -> Self {
        Self::from_host().unwrap()
    }

    /// Returns the target triple for the current host as a string.
    pub fn host_str() -> String {
        Self::from_host().unwrap().as_str()
    }

    /// Returns whether the target triple is the same as the current host.
    pub fn is_host(&self) -> bool {
        let host = Self::from_host().unwrap();
        self.arch == host.arch && self.vendor == host.vendor && self.os == host.os
    }

    /// Get the architecture of the target.
    pub fn arch(&self) -> Arch {
        self.arch
    }

    /// Get the vendor of the target.
    pub fn vendor(&self) -> Vendor {
        self.vendor
    }

    /// Get the target operating system.
    pub fn os(&self) -> TargetOS {
        self.os
    }
}

impl Debug for TargetTriple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_str())
    }
}
