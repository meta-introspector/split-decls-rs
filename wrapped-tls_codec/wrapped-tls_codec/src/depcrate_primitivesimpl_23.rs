// Generated macro for impl_23 (impl)
macro_rules! Depcrate_primitivesimpl_23 {
() => {
// Module: crate::primitives
// Provides: {"impl_23"}
// Dependencies: {}
impl < T : Serialize > Serialize for Option < T > { # [cfg (feature = "std")] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { match self { Some (e) => { let written = writer . write (& [1]) ? ; debug_assert_eq ! (written , 1) ; e . tls_serialize (writer) . map (| l | l + 1) } None => { writer . write_all (& [0]) ? ; Ok (1) } } } }
};
}
