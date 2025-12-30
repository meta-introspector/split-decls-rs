// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl < T , S > Layer < S > for & T where T : ? Sized + Layer < S > , { type Service = T :: Service ; fn layer (& self , inner : S) -> Self :: Service { (* * self) . layer (inner) } }
};
}
