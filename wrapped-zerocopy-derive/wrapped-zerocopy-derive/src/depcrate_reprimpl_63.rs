// Generated macro for impl_63 (impl)
macro_rules! Depcrate_reprimpl_63 {
() => {
// Module: crate::repr
// Provides: {"impl_63"}
// Dependencies: {}
impl From < Spanned < FromAttrsError > > for Error { fn from (err : Spanned < FromAttrsError >) -> Error { let Spanned { t : err , span } = err ; match err { FromAttrsError :: FromRawReprs (FromRawReprsError :: Single (_err @ UnsupportedReprError ,)) => Error :: new (span , "unsupported representation hint for the decorated type") , FromAttrsError :: FromRawReprs (FromRawReprsError :: Conflict) => { Error :: new (span , "this conflicts with another representation hint") } FromAttrsError :: Unrecognized => Error :: new (span , "unrecognized representation hint") , } } }
};
}
