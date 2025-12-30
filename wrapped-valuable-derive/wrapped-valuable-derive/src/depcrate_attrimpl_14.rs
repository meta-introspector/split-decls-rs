// Generated macro for impl_14 (impl)
macro_rules! Depcrate_attrimpl_14 {
() => {
// Module: crate::attr
// Provides: {"impl_14"}
// Dependencies: {}
impl From < & Fields > for Position { fn from (meta : & Fields) -> Self { match meta { Fields :: Named (..) => Position :: NamedField , Fields :: Unnamed (..) | Fields :: Unit => Position :: UnnamedField , } } }
};
}
