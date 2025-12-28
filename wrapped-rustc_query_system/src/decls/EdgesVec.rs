macro_rules! EdgesVec {
    () => {
        # [derive (Default , Debug)] pub (crate) struct EdgesVec { max : u32 , edges : SmallVec < [DepNodeIndex ; EdgesVec :: INLINE_CAPACITY] > , }
    };
}

EdgesVec!()