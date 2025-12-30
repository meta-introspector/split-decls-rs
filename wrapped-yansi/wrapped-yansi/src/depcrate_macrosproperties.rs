// Generated macro for properties (macro)
macro_rules! Depcrate_macrosproperties {
() => {
// Module: crate::macros
// Provides: {"properties"}
// Dependencies: {}
macro_rules ! properties { ($ ([$ ($ qual : tt) *]) ? $ cont : ident ($ r : ty) -> $ R : ty) => (fg ! ($ ([$ ($ qual) *]) ? $ cont ($ r) -> $ R) ; bg ! ($ ([$ ($ qual) *]) ? $ cont ($ r) -> $ R) ; attr ! ($ ([$ ($ qual) *]) ? $ cont ($ r) -> $ R) ; quirk ! ($ ([$ ($ qual) *]) ? $ cont ($ r) -> $ R) ; whenever ! ($ ([$ ($ qual) *]) ? $ cont ($ r) -> $ R) ;) }
};
}
