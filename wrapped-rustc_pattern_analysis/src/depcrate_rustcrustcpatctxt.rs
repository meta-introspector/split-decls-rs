// Generated macro for RustcPatCtxt (struct)
macro_rules! Depcrate_rustcRustcPatCtxt {
() => {
// Module: crate::rustc
// Provides: {"RustcPatCtxt"}
// Dependencies: {}
# [derive (Clone)] pub struct RustcPatCtxt < 'p , 'tcx : 'p > { pub tcx : TyCtxt < 'tcx > , pub typeck_results : & 'tcx ty :: TypeckResults < 'tcx > , # [doc = " The module in which the match occurs. This is necessary for"] # [doc = " checking inhabited-ness of types because whether a type is (visibly)"] # [doc = " inhabited can depend on whether it was defined in the current module or"] # [doc = " not. E.g., `struct Foo { _private: ! }` cannot be seen to be empty"] # [doc = " outside its module and should not be matchable with an empty match statement."] pub module : DefId , pub typing_env : ty :: TypingEnv < 'tcx > , # [doc = " To allocate the result of `self.ctor_sub_tys()`"] pub dropless_arena : & 'p DroplessArena , # [doc = " Lint level at the match."] pub match_lint_level : HirId , # [doc = " The span of the whole match, if applicable."] pub whole_match_span : Option < Span > , # [doc = " Span of the scrutinee."] pub scrut_span : Span , # [doc = " Only produce `NON_EXHAUSTIVE_OMITTED_PATTERNS` lint on refutable patterns."] pub refutable : bool , # [doc = " Whether the data at the scrutinee is known to be valid. This is false if the scrutinee comes"] # [doc = " from a union field, a pointer deref, or a reference deref (pending opsem decisions)."] pub known_valid_scrutinee : bool , pub internal_state : RustcPatCtxtState , }
};
}
