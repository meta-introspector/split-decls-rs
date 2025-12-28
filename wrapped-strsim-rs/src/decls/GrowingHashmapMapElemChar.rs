macro_rules! GrowingHashmapMapElemChar {
    () => {
        # [derive (Default , Clone)] struct GrowingHashmapMapElemChar < ValueType > { key : u32 , value : ValueType , }
    };
}

GrowingHashmapMapElemChar!();