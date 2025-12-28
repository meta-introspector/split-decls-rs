macro_rules! Mmap {
    () => {
        # [cfg (any (miri , target_arch = "wasm32"))] pub struct Mmap (Vec < u8 >) ;
    };
}

Mmap!();