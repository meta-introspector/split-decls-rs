// Generated macro for impl_49 (impl)
macro_rules! Depcrate_first_passimpl_49 {
() => {
// Module: crate::first_pass
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , ApiStability > for [weedle :: Definition < 'src >] { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , stability : ApiStability ,) -> Result < () > { for def in self { def . first_pass (record , stability) ? ; } Ok (()) } }
};
}
