// Generated macro for banner (function)
macro_rules! Depcratebanner {
() => {
// Module: crate
// Provides: {"banner"}
// Dependencies: {}
fn banner () { info ! ("This is varisat {}" , env ! ("VARISAT_VERSION")) ; info ! ("  {} build - {}" , env ! ("VARISAT_PROFILE") , env ! ("VARISAT_RUSTC_VERSION")) ; }
};
}
