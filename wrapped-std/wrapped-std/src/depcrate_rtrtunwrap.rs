// Generated macro for rtunwrap (macro)
macro_rules! Depcrate_rtrtunwrap {
() => {
// Module: crate::rt
// Provides: {"rtunwrap"}
// Dependencies: {}
macro_rules ! rtunwrap { ($ ok : ident , $ e : expr) => { match $ e { $ ok (v) => v , ref err => { let err = err . as_ref () . map (drop) ; rtabort ! (concat ! ("unwrap failed: " , stringify ! ($ e) , " = {:?}") , err) } } } ; }
};
}
