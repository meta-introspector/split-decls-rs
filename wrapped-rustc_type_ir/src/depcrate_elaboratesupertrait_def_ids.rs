// Generated macro for supertrait_def_ids (function)
macro_rules! Depcrate_elaboratesupertrait_def_ids {
() => {
// Module: crate::elaborate
// Provides: {"supertrait_def_ids"}
// Dependencies: {}
# [doc = " Computes the def-ids of the transitive supertraits of `trait_def_id`. This (intentionally)"] # [doc = " does not compute the full elaborated super-predicates but just the set of def-ids. It is used"] # [doc = " to identify which traits may define a given associated type to help avoid cycle errors,"] # [doc = " and to make size estimates for vtable layout computation."] pub fn supertrait_def_ids < I : Interner > (cx : I , trait_def_id : I :: TraitId ,) -> impl Iterator < Item = I :: TraitId > { let mut set = HashSet :: default () ; let mut stack = vec ! [trait_def_id] ; set . insert (trait_def_id) ; std :: iter :: from_fn (move | | { let trait_def_id = stack . pop () ? ; for (predicate , _) in cx . explicit_super_predicates_of (trait_def_id) . iter_identity () { if let ty :: ClauseKind :: Trait (data) = predicate . kind () . skip_binder () && set . insert (data . def_id ()) { stack . push (data . def_id ()) ; } } Some (trait_def_id) }) }
};
}
