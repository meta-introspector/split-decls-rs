// Generated macro for impl_143 (impl)
macro_rules! Depcrate_helpersimpl_143 {
() => {
// Module: crate::helpers
// Provides: {"impl_143"}
// Dependencies: {}
impl < T > DebugUnwrapOr < T > for Option < T > { # [inline] fn debug_unwrap_or (self , gigo_value : T) -> T { match self { Some (x) => x , None => { debug_assert ! (false , "debug_unwrap_or called on a None value") ; gigo_value } } } }
};
}
