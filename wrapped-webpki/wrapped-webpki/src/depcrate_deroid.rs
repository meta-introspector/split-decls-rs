// Generated macro for oid (macro)
macro_rules! Depcrate_deroid {
() => {
// Module: crate::der
// Provides: {"oid"}
// Dependencies: {}
macro_rules ! oid { ($ first : expr , $ second : expr , $ ($ tail : expr) ,*) => ([(40 * $ first) + $ second , $ ($ tail) ,*]) }
};
}
