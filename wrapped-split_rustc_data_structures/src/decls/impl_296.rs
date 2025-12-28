macro_rules! deps {
    () => {
        Mmap!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        # [cfg (not (any (miri , target_arch = "wasm32")))] impl Mmap { # [doc = " # Safety"] # [doc = ""] # [doc = " The given file must not be mutated (i.e., not written, not truncated, ...) until the mapping is closed."] # [doc = ""] # [doc = " However in practice most callers do not ensure this, so uses of this function are likely unsound."] # [inline] pub unsafe fn map (file : File) -> io :: Result < Self > { unsafe { memmap2 :: MmapOptions :: new () . map_copy_read_only (& file) . map (Mmap) } } }
    };
}

impl_296!()