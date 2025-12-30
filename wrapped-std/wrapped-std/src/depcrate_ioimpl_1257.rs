// Generated macro for impl_1257 (impl)
macro_rules! Depcrate_ioimpl_1257 {
() => {
// Module: crate::io
// Provides: {"impl_1257"}
// Dependencies: {}
impl < T > SizeHint for Take < T > { # [inline] fn lower_bound (& self) -> usize { cmp :: min (SizeHint :: lower_bound (& self . inner) as u64 , self . limit) as usize } # [inline] fn upper_bound (& self) -> Option < usize > { match SizeHint :: upper_bound (& self . inner) { Some (upper_bound) => Some (cmp :: min (upper_bound as u64 , self . limit) as usize) , None => self . limit . try_into () . ok () , } } }
};
}
