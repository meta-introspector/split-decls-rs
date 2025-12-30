// Generated macro for impl_143 (impl)
macro_rules! Depcrate_tendrilimpl_143 {
() => {
// Module: crate::tendril
// Provides: {"impl_143"}
// Dependencies: {}
impl < F , A > strfmt :: Debug for Tendril < F , A > where F : fmt :: SliceFormat + Default + strfmt :: Debug , < F as fmt :: SliceFormat > :: Slice : strfmt :: Debug , A : Atomicity , { # [inline] fn fmt (& self , f : & mut strfmt :: Formatter) -> strfmt :: Result { let kind = match self . ptr . get () . get () { p if p <= MAX_INLINE_TAG => "inline" , p if p & 1 == 1 => "shared" , _ => "owned" , } ; try ! (write ! (f , "Tendril<{:?}>({}: " , < F as Default >:: default () , kind)) ; try ! (<< F as fmt :: SliceFormat >:: Slice as strfmt :: Debug >:: fmt (&** self , f)) ; write ! (f , ")") } }
};
}
