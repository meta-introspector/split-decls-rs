use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl VisitMut for SyscallAstTransformer { fn visit_expr_mut (& mut self , expr : & mut Expr) { match expr { Expr :: Call (call) => { if let Expr :: Path (path_expr) = & * call . func { if let Some (wrapper) = self . is_syscall_function (& path_expr . path) . cloned () { let wrapped = self . wrap_syscall (expr , & wrapper) ; if let Ok (new_expr) = syn :: parse2 :: < Expr > (wrapped) { * expr = new_expr ; return ; } } } } Expr :: MethodCall (method_call) => { let method_name = method_call . method . to_string () ; if let Some (wrapper) = self . interceptor . syscall_mappings . get (& method_name) . cloned () { let wrapped = self . wrap_syscall (expr , & wrapper) ; if let Ok (new_expr) = syn :: parse2 :: < Expr > (wrapped) { * expr = new_expr ; return ; } } } _ => { } } syn :: visit_mut :: visit_expr_mut (self , expr) ; } }
}