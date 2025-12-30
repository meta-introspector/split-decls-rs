// Generated macro for impl_38 (impl)
macro_rules! Depcrate_layoutimpl_38 {
() => {
// Module: crate::layout
// Provides: {"impl_38"}
// Dependencies: {}
impl Byte { const UNINIT : u16 = 256 ; # [inline] fn new (range : RangeInclusive < u8 >) -> Self { let start : u16 = (* range . start ()) . into () ; let end : u16 = (* range . end ()) . into () ; Byte { start , end : end + 1 } } # [inline] fn from_val (val : u8) -> Self { let val : u16 = val . into () ; Byte { start : val , end : val + 1 } } # [inline] fn uninit () -> Byte { Byte { start : 0 , end : Self :: UNINIT + 1 } } # [inline] fn is_empty (& self) -> bool { self . start == self . end } # [inline] fn contains_uninit (& self) -> bool { self . start <= Self :: UNINIT && Self :: UNINIT < self . end } }
};
}
