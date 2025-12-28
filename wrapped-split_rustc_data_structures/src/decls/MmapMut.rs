macro_rules! MmapMut {
    () => {
        # [cfg (any (miri , target_arch = "wasm32"))] pub struct MmapMut (Vec < u8 >) ;
    };
}

MmapMut!();