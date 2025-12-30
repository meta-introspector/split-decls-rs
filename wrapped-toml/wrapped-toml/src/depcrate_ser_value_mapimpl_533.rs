// Generated macro for impl_533 (impl)
macro_rules! Depcrate_ser_value_mapimpl_533 {
() => {
// Module: crate::ser::value::map
// Provides: {"impl_533"}
// Dependencies: {}
impl < 'd > SerializeDatetime < 'd > { pub (crate) fn new (dst : & 'd mut String) -> Self { Self { dst , inner : toml_datetime :: ser :: DatetimeSerializer :: new () , } } }
};
}
