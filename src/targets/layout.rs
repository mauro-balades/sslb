use crate::targets::triple::TargetTriple;
use crate::targets::triple::Arch;


pub struct DataLayout {
    /// The size of a pointer.
    pub pointer_size: u64,
    /// The alignment of a pointer.
    pub pointer_align: u64,
    /// The size of a `usize`.
    pub usize_size: u64,
    /// The alignment of a `usize`.
    pub usize_align: u64,
    /// The size of an `isize`.
    pub isize_size: u64,
    /// The alignment of an `isize`.
    pub isize_align: u64,
    /// The size of a `u8`.
    pub u8_size: u64,
    /// The alignment of a `u8`.
    pub u8_align: u64,
    /// The size of an `i8`.
    pub i8_size: u64,
    /// The alignment of an `i8`.
    pub i8_align: u64,
    /// The size of a `u16`.
    pub u16_size: u64,
    /// The alignment of a `u16`.
    pub u16_align: u64,
    /// The size of an `i16`.
    pub i16_size: u64,
    /// The alignment of an `i16`.
    pub i16_align: u64,
    /// The size of a `u32`.
    pub u32_size: u64,
    /// The alignment of a `u32`.
    pub u32_align: u64,
    /// The size of an `i32`.
    pub i32_size: u64,
    /// The alignment of an `i32`.
    pub i32_align: u64,
    /// The size of a `u64`.
    pub u64_size: u64,
    /// The alignment of a `u64`.
    pub u64_align: u64,
    /// The size of an `i64`.
    pub i64_size: u64,
    /// The alignment of an `i64`.
    pub i64_align: u64,
    /// The size of a `u128`.
    pub u128_size: u64,
    /// The alignment of a `u128`.
    pub u128_align: u64,
    /// The size of an `i128`.
    pub i128_size: u64,
    /// The alignment of an `i128`.
    pub i128_align: u64,
    /// The size of a `f32`.
    pub f32_size: u64,
    /// The alignment of a `f32`.
    pub f32_align: u64,
    /// The size of a `f64`.
    pub f64_size: u64,
    /// The alignment of a `f64`.
    pub f64_align: u64,
}

impl DataLayout {
    /// Creates a new `DataLayout`.
    pub fn new(pointer_size: u64) -> Self {
        Self {
            pointer_size,
            pointer_align: pointer_size,
            usize_size: pointer_size,
            usize_align: pointer_size,
            isize_size: pointer_size,
            isize_align: pointer_size,
            u8_size: 1,
            u8_align: 1,
            i8_size: 1,
            i8_align: 1,
            u16_size: 2,
            u16_align: 2,
            i16_size: 2,
            i16_align: 2,
            u32_size: 4,
            u32_align: 4,
            i32_size: 4,
            i32_align: 4,
            u64_size: 8,
            u64_align: 8,
            i64_size: 8,
            i64_align: 8,
            u128_size: 16,
            u128_align: 16,
            i128_size: 16,
            i128_align: 16,
            f32_size: 4,
            f32_align: 4,
            f64_size: 8,
            f64_align: 8,
        }
    }

    pub fn from_triple(triple: &TargetTriple) -> Self {
        match triple.arch() {
            Arch::X86_64 => Self::new_x86_64(),
            Arch::X86 => Self::new_x86(),
            Arch::Arm => Self::new_arm(),
            Arch::Aarch64 => Self::new_aarch64(),
            _ => Self::default(),
        }
    }

    pub fn new_x86_64() -> Self {
        Self::new(8)
    }

    pub fn new_x86() -> Self {
        Self::new(4)
    }

    pub fn new_arm() -> Self {
        Self::new(4)
    }

    pub fn new_aarch64() -> Self {
        Self::new(8)
    }

    pub fn default() -> Self {
        Self::new_x86_64()
    }

    /// Returns the size of a pointer.
    pub fn pointer_size(&self) -> u64 {
        self.pointer_size
    }

    /// Returns the alignment of a pointer.
    pub fn pointer_align(&self) -> u64 {
        self.pointer_align
    }

    /// Returns the size of a `usize`.
    pub fn usize_size(&self) -> u64 {
        self.usize_size
    }

    /// Returns the alignment of a `usize`.
    pub fn usize_align(&self) -> u64 {
        self.usize_align
    }

    /// Returns the size of an `isize`.
    pub fn isize_size(&self) -> u64 {
        self.isize_size
    }

    /// Returns the alignment of an `isize`.
    pub fn isize_align(&self) -> u64 {
        self.isize_align
    }

    /// Returns the size of a `u8`.
    pub fn u8_size(&self) -> u64 {
        self.u8_size
    }

    /// Returns the alignment of a `u8`.
    pub fn u8_align(&self) -> u64 {
        self.u8_align
    }

    /// Returns the size of an `i8`.
    pub fn i8_size (&self) -> u64 {
        self.i8_size
    }

    /// Returns the alignment of an `i8`.
    pub fn i8_align(&self) -> u64 {
        self.i8_align
    }

    /// Returns the size of a `u16`.
    pub fn u16_size(&self) -> u64 {
        self.u16_size
    }

    /// Returns the alignment of a `u16`.
    pub fn u16_align(&self) -> u64 {
        self.u16_align
    }

    /// Returns the size of an `i16`.
    pub fn i16_size(&self) -> u64 {
        self.i16_size
    }

    /// Returns the alignment of an `i16`.
    pub fn i16_align(&self) -> u64 {
        self.i16_align
    }

    /// Returns the size of a `u32`.
    pub fn u32_size(&self) -> u64 {
        self.u32_size
    }

    /// Returns the alignment of a `u32`.
    pub fn u32_align(&self) -> u64 {
        self.u32_align
    }

    /// Returns the size of an `i32`.
    pub fn i32_size(&self) -> u64 {
        self.i32_size
    }

    /// Returns the alignment of an `i32`.
    pub fn i32_align(&self) -> u64 {
        self.i32_align
    }

    /// Returns the size of a `u64`.
    pub fn u64_size(&self) -> u64 {
        self.u64_size
    }

    /// Returns the alignment of a `u64`.
    pub fn u64_align(&self) -> u64 {
        self.u64_align
    }

    /// Returns the size of an `i64`.
    pub fn i64_size(&self) -> u64 {
        self.i64_size
    }

    /// Returns the alignment of an `i64`.
    pub fn i64_align(&self) -> u64 {
        self.i64_align
    }

    /// Returns the size of a `u128`.
    pub fn u128_size(&self) -> u64 {
        self.u128_size
    }

    /// Returns the alignment of a `u128`.
    pub fn u128_align(&self) -> u64 {
        self.u128_align
    }

    /// Returns the size of an `i128`.
    pub fn i128_size(&self) -> u64 {
        self.i128_size
    }

    /// Returns the alignment of an `i128`.
    pub fn i128_align(&self) -> u64 {
        self.i128_align
    }

    /// Returns the size of a `f32`.
    pub fn f32_size(&self) -> u64 {
        self.f32_size
    }

    /// Returns the alignment of a `f32`.
    pub fn f32_align(&self) -> u64 {
        self.f32_size
    }

    /// Returns the size of a `f64`.
    pub fn f64_size(&self) -> u64 {
        self.f64_size
    }

    /// Returns the alignment of a `f64`.
    pub fn f64_align(&self) -> u64 {
        self.f64_align
    }
}
