// Generated macro for impl_80 (impl)
macro_rules! Depcrate_recent_blockhashesimpl_80 {
() => {
// Module: crate::recent_blockhashes
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a > FromIterator < IterItem < 'a > > for RecentBlockhashes { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = IterItem < 'a > > , { let mut new = Self :: default () ; for i in iter { new . 0 . push (Entry :: new (i . 1 , i . 2)) } new } }
};
}
