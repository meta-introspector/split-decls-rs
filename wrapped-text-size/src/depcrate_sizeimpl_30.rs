// Generated macro for impl_30 (impl)
macro_rules! Depcrate_sizeimpl_30 {
() => {
// Module: crate::size
// Provides: {"impl_30"}
// Dependencies: {}
impl TryFrom < usize > for TextSize { type Error = TryFromIntError ; # [inline] fn try_from (value : usize) -> Result < Self , TryFromIntError > { Ok (u32 :: try_from (value) ? . into ()) } }
};
}
