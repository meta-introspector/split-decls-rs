// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl < T > Entry < T > { fn get_value_cell (& self) -> Option < & UnsafeCell < MaybeUninit < T > > > { self . present . load (Ordering :: Acquire) . then_some (& self . value) } # [doc = " # Safety"] # [doc = " The caller must guarantee that there are no concurent mutable accesses into"] # [doc = " this entry's value."] unsafe fn as_ref < 'a > (& self) -> Option < & 'a T > { self . get_value_cell () . map (| cell | unsafe { (& * cell . get ()) . assume_init_ref () }) } }
};
}
