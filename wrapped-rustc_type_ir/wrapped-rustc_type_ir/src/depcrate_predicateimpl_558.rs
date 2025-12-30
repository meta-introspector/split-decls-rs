// Generated macro for impl_558 (impl)
macro_rules! Depcrate_predicateimpl_558 {
() => {
// Module: crate::predicate
// Provides: {"impl_558"}
// Dependencies: {}
impl < I : Interner > From < ty :: AliasTy < I > > for AliasTerm < I > { fn from (ty : ty :: AliasTy < I >) -> Self { AliasTerm { args : ty . args , def_id : ty . def_id , _use_alias_term_new_instead : () } } }
};
}
