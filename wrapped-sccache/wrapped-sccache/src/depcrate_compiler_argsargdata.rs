// Generated macro for ArgData (macro)
macro_rules! Depcrate_compiler_argsArgData {
() => {
// Module: crate::compiler::args
// Provides: {"ArgData"}
// Dependencies: {}
macro_rules ! ArgData { { __matchify $ var : ident $ fn : ident ($ ($ fnarg : ident) *) ($ ($ arms : tt) *) } => { match $ var { $ ($ arms) * } } ; { __matchify $ var : ident $ fn : ident ($ ($ fnarg : ident) *) ($ ($ arms : tt) *) $ x : ident , $ ($ rest : tt) * } => { ArgData ! { __matchify $ var $ fn ($ ($ fnarg) *) ($ ($ arms) * ArgData ::$ x => () .$ fn ($ ($ fnarg) *) ,) $ ($ rest) * } } ; { __matchify $ var : ident $ fn : ident ($ ($ fnarg : ident) *) ($ ($ arms : tt) *) $ x : ident ($ y : ty) , $ ($ rest : tt) * } => { ArgData ! { __matchify $ var $ fn ($ ($ fnarg) *) ($ ($ arms) * ArgData ::$ x (inner) => inner .$ fn ($ ($ fnarg) *) ,) $ ($ rest) * } } ; { __impl $ ($ tok : tt) + } => { impl IntoArg for ArgData { fn into_arg_os_string (self) -> OsString { ArgData ! { __matchify self into_arg_os_string () () $ ($ tok) + } } fn into_arg_string (self , transformer : PathTransformerFn <'_ >) -> ArgToStringResult { ArgData ! { __matchify self into_arg_string (transformer) () $ ($ tok) + } } } } ; { pub $ ($ tok : tt) + } => { # [derive (Clone , Debug , PartialEq , Eq)] pub enum ArgData { $ ($ tok) + } ArgData ! { __impl $ ($ tok) + } } ; { $ ($ tok : tt) + } => { # [derive (Clone , Debug , PartialEq)] # [allow (clippy :: enum_variant_names)] enum ArgData { $ ($ tok) + } ArgData ! { __impl $ ($ tok) + } } ; }
};
}
