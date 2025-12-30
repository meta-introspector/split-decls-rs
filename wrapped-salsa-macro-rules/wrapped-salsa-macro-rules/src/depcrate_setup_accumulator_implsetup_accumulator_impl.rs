// Generated macro for setup_accumulator_impl (macro)
macro_rules! Depcrate_setup_accumulator_implsetup_accumulator_impl {
() => {
// Module: crate::setup_accumulator_impl
// Provides: {"setup_accumulator_impl"}
// Dependencies: {}
# [doc = " Macro for setting up a function that must intern its arguments."] # [macro_export] macro_rules ! setup_accumulator_impl { (Struct : $ Struct : ident , unused_names : [$ zalsa : ident , $ zalsa_struct : ident , $ CACHE : ident , $ ingredient : ident ,]) => { # [allow (clippy :: all)] # [allow (dead_code)] const _ : () = { use salsa :: plumbing as $ zalsa ; use salsa :: plumbing :: accumulator as $ zalsa_struct ; impl $ zalsa :: HasJar for $ Struct { type Jar = $ zalsa_struct :: JarImpl <$ Struct >; const KIND : $ zalsa :: JarKind = $ zalsa :: JarKind :: Struct ; } $ zalsa :: register_jar ! { $ zalsa :: ErasedJar :: erase ::<$ Struct > () } fn $ ingredient (zalsa : &$ zalsa :: Zalsa) -> &$ zalsa_struct :: IngredientImpl <$ Struct > { static $ CACHE : $ zalsa :: IngredientCache <$ zalsa_struct :: IngredientImpl <$ Struct >> = $ zalsa :: IngredientCache :: new () ; unsafe { $ CACHE . get_or_create (zalsa , || { zalsa . lookup_jar_by_type ::<$ zalsa_struct :: JarImpl <$ Struct >> () }) } } impl $ zalsa :: Accumulator for $ Struct { const DEBUG_NAME : &'static str = stringify ! ($ Struct) ; fn accumulate < Db > (self , db : & Db) where Db : ? Sized + $ zalsa :: Database , { let (zalsa , zalsa_local) = db . zalsas () ; $ ingredient (zalsa) . push (zalsa_local , self) ; } } } ; } ; }
};
}
