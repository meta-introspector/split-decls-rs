// Generated macro for collect_nonexhaustive_missing_variants (function)
macro_rules! Depcrate_lintscollect_nonexhaustive_missing_variants {
() => {
// Module: crate::lints
// Provides: {"collect_nonexhaustive_missing_variants"}
// Dependencies: {}
# [doc = " Traverse the patterns to collect any variants of a non_exhaustive enum that fail to be mentioned"] # [doc = " in a given column."] # [instrument (level = "debug" , skip (cx) , ret)] fn collect_nonexhaustive_missing_variants < 'p , 'tcx > (cx : & RustcPatCtxt < 'p , 'tcx > , column : & PatternColumn < 'p , RustcPatCtxt < 'p , 'tcx > > ,) -> Result < Vec < WitnessPat < 'p , 'tcx > > , ErrorGuaranteed > { let Some (& ty) = column . head_ty () else { return Ok (Vec :: new ()) ; } ; let set = column . analyze_ctors (cx , & ty) ? ; if set . present . is_empty () { return Ok (Vec :: new ()) ; } let mut witnesses = Vec :: new () ; if cx . is_foreign_non_exhaustive_enum (ty) { witnesses . extend (set . missing . into_iter () . filter (| c | ! matches ! (c , Constructor :: Hidden | Constructor :: NonExhaustive)) . map (| missing_ctor | WitnessPat :: wild_from_ctor (cx , missing_ctor , ty)) ,) } for ctor in set . present { let specialized_columns = column . specialize (cx , & ty , & ctor) ; let wild_pat = WitnessPat :: wild_from_ctor (cx , ctor , ty) ; for (i , col_i) in specialized_columns . iter () . enumerate () { let wits_for_col_i = collect_nonexhaustive_missing_variants (cx , col_i) ? ; for wit in wits_for_col_i { let mut pat = wild_pat . clone () ; pat . fields [i] = wit ; witnesses . push (pat) ; } } } Ok (witnesses) }
};
}
