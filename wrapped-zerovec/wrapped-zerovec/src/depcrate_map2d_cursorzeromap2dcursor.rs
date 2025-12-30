// Generated macro for ZeroMap2dCursor (struct)
macro_rules! Depcrate_map2d_cursorZeroMap2dCursor {
() => {
// Module: crate::map2d::cursor
// Provides: {"ZeroMap2dCursor"}
// Dependencies: {}
# [doc = " An intermediate state of queries over [`ZeroMap2d`] and [`ZeroMap2dBorrowed`]."] pub struct ZeroMap2dCursor < 'l , 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > , K1 : ZeroMapKV < 'a > , V : ZeroMapKV < 'a > , K0 : ? Sized , K1 : ? Sized , V : ? Sized , { keys0 : & 'l K0 :: Slice , joiner : & 'l ZeroSlice < u32 > , keys1 : & 'l K1 :: Slice , values : & 'l V :: Slice , key0_index : usize , }
};
}
