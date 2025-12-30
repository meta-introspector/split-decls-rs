// Generated macro for shift_region (function)
macro_rules! Depcrate_foldshift_region {
() => {
// Module: crate::fold
// Provides: {"shift_region"}
// Dependencies: {}
pub fn shift_region < I : Interner > (cx : I , region : I :: Region , amount : u32) -> I :: Region { match region . kind () { ty :: ReBound (debruijn , br) if amount > 0 => { Region :: new_bound (cx , debruijn . shifted_in (amount) , br) } _ => region , } }
};
}
