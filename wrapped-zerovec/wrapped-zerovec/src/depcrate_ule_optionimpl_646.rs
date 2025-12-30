// Generated macro for impl_646 (impl)
macro_rules! Depcrate_ule_optionimpl_646 {
() => {
// Module: crate::ule::option
// Provides: {"impl_646"}
// Dependencies: {}
impl < U : Copy > OptionULE < U > { # [doc = " Obtain this as an `Option<T>`"] pub fn get (self) -> Option < U > { if self . 0 { unsafe { Some (self . 1 . assume_init ()) } } else { None } } # [doc = " Construct an `OptionULE<U>` from an equivalent `Option<T>`"] pub fn new (opt : Option < U >) -> Self { if let Some (inner) = opt { Self (true , MaybeUninit :: new (inner)) } else { Self (false , MaybeUninit :: zeroed ()) } } }
};
}
