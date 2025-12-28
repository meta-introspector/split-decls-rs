macro_rules! deps {
    () => {
        ReachableContext!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for ReachableContext < 'tcx > { fn visit_nested_body (& mut self , body : hir :: BodyId) { let old_maybe_typeck_results = self . maybe_typeck_results . replace (self . tcx . typeck_body (body)) ; let body = self . tcx . hir_body (body) ; self . visit_body (body) ; self . maybe_typeck_results = old_maybe_typeck_results ; } fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) { let res = match expr . kind { hir :: ExprKind :: Path (ref qpath) => { Some (self . typeck_results () . qpath_res (qpath , expr . hir_id)) } hir :: ExprKind :: MethodCall (..) => { self . typeck_results () . type_dependent_def (expr . hir_id) . map (| (kind , def_id) | Res :: Def (kind , def_id)) } hir :: ExprKind :: Closure (& hir :: Closure { def_id , .. }) => { self . reachable_symbols . insert (def_id) ; None } _ => None , } ; if let Some (res) = res { self . propagate_item (res) ; } intravisit :: walk_expr (self , expr) } fn visit_inline_asm (& mut self , asm : & 'tcx hir :: InlineAsm < 'tcx > , id : hir :: HirId) { for (op , _) in asm . operands { if let hir :: InlineAsmOperand :: SymStatic { def_id , .. } = op && let Some (def_id) = def_id . as_local () { self . reachable_symbols . insert (def_id) ; } } intravisit :: walk_inline_asm (self , asm , id) ; } }
    };
}

impl_308!()