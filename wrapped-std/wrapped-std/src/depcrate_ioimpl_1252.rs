// Generated macro for impl_1252 (impl)
macro_rules! Depcrate_ioimpl_1252 {
() => {
// Module: crate::io
// Provides: {"impl_1252"}
// Dependencies: {}
impl < T , U > SizeHint for Chain < T , U > { # [inline] fn lower_bound (& self) -> usize { SizeHint :: lower_bound (& self . first) + SizeHint :: lower_bound (& self . second) } # [inline] fn upper_bound (& self) -> Option < usize > { match (SizeHint :: upper_bound (& self . first) , SizeHint :: upper_bound (& self . second)) { (Some (first) , Some (second)) => first . checked_add (second) , _ => None , } } }
};
}
