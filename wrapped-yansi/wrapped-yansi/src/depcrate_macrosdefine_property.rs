// Generated macro for define_property (macro)
macro_rules! Depcrate_macrosdefine_property {
() => {
// Module: crate::macros
// Provides: {"define_property"}
// Dependencies: {}
macro_rules ! define_property { ([$ d : tt] $ (# [$ attr : meta]) * $ kind : ident ($ A : ty) { $ ($ (# [$ pattr : meta]) * $ prop : ident => $ V : path $ ([$ ($ a : tt) *]) ?) ,* $ (,) ? }) => { macro_rules ! $ kind { ($ d ([$ d ($ qual : tt) *]) ? $ cont : ident ($ r : ty) -> $ R : ty) => ($ cont ! ([$ d ($ d ($ qual) *) ?] $ (# [$ attr]) * $ r , $ R , $ kind ($ A)) ; $ ($ cont ! ([$ d ($ d ($ qual) *) ?] $ r , $ R , $ kind , $ (# [$ pattr]) * $ prop => $ V $ ([$ ($ a) *]) ?) ;) *) } } ; ($ (# [$ attr : meta]) * $ kind : ident ($ A : ty)) => { define_property ! ([$] $ (# [$ attr]) * $ kind ($ A) { }) ; } ; ($ ($ t : tt) *) => { define_property ! ([$] $ ($ t) *) ; } }
};
}
