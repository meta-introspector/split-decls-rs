use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn check_c_variadic_abi (tcx : TyCtxt < '_ > , decl : & hir :: FnDecl < '_ > , abi : ExternAbi , span : Span) { if ! decl . c_variadic { return ; } match abi . supports_c_variadic () { CVariadicStatus :: Stable => { } CVariadicStatus :: NotSupported => { tcx . dcx () . create_err (errors :: VariadicFunctionCompatibleConvention { span , convention : & format ! ("{abi}") , }) . emit () ; } CVariadicStatus :: Unstable { feature } => { if ! tcx . features () . enabled (feature) { feature_err (& tcx . sess , feature , span , format ! ("C-variadic functions with the {abi} calling convention are unstable") ,) . emit () ; } } } }
}