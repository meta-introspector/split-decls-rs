// Generated macro for impl_601 (impl)
macro_rules! Depcrate_sso_mapimpl_601 {
() => {
// Module: crate::sso::map
// Provides: {"impl_601"}
// Dependencies: {}
impl < 'a , K , V > Index < & 'a K > for SsoHashMap < K , V > where K : Eq + Hash , { type Output = V ; # [inline] fn index (& self , key : & K) -> & V { self . get (key) . expect ("no entry found for key") } }
};
}
