
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
}
