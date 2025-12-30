// Generated macro for impl_90 (impl)
macro_rules! Depcrate_cross_crate_inlineimpl_90 {
() => {
// Module: crate::cross_crate_inline
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for CostChecker < '_ , 'tcx > { fn visit_statement (& mut self , statement : & Statement < 'tcx > , _ : Location) { match statement . kind { StatementKind :: StorageLive (_) | StatementKind :: StorageDead (_) | StatementKind :: Deinit (_) | StatementKind :: Nop => { } _ => self . statements += 1 , } } fn visit_terminator (& mut self , terminator : & Terminator < 'tcx > , _ : Location) { let tcx = self . tcx ; match terminator . kind { TerminatorKind :: Drop { ref place , unwind , .. } => { let ty = place . ty (self . callee_body , tcx) . ty ; if ! ty . is_trivially_pure_clone_copy () { self . calls += 1 ; if let UnwindAction :: Cleanup (_) = unwind { self . landing_pads += 1 ; } } } TerminatorKind :: Call { ref func , unwind , .. } => { if let Some ((fn_def_id , _)) = func . const_fn_def () && self . tcx . has_attr (fn_def_id , sym :: rustc_intrinsic) { return ; } self . calls += 1 ; if let UnwindAction :: Cleanup (_) = unwind { self . landing_pads += 1 ; } } TerminatorKind :: Assert { unwind , .. } => { self . calls += 1 ; if let UnwindAction :: Cleanup (_) = unwind { self . landing_pads += 1 ; } } TerminatorKind :: UnwindResume => self . resumes += 1 , TerminatorKind :: InlineAsm { unwind , .. } => { self . statements += 1 ; if let UnwindAction :: Cleanup (_) = unwind { self . landing_pads += 1 ; } } TerminatorKind :: Return => { } _ => self . statements += 1 , } } }
};
}
