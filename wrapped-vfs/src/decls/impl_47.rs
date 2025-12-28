macro_rules! deps {
    () => {
        FileId!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl FileId { const MAX : u32 = 0x7fff_ffff ; # [inline] pub const fn from_raw (raw : u32) -> FileId { assert ! (raw <= Self :: MAX) ; FileId (raw) } # [inline] pub const fn index (self) -> u32 { self . 0 } }
    };
}

impl_47!();