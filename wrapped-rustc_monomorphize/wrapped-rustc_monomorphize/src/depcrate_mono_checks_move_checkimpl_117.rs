// Generated macro for impl_117 (impl)
macro_rules! Depcrate_mono_checks_move_checkimpl_117 {
() => {
// Module: crate::mono_checks::move_check
// Provides: {"impl_117"}
// Dependencies: {}
impl < 'tcx > MirVisitor < 'tcx > for MoveCheckVisitor < 'tcx > { fn visit_terminator (& mut self , terminator : & mir :: Terminator < 'tcx > , location : Location) { match terminator . kind { mir :: TerminatorKind :: Call { ref func , ref args , ref fn_span , .. } | mir :: TerminatorKind :: TailCall { ref func , ref args , ref fn_span } => { let callee_ty = func . ty (self . body , self . tcx) ; let callee_ty = self . monomorphize (callee_ty) ; self . check_fn_args_move_size (callee_ty , args , * fn_span , location) ; } _ => { } } } fn visit_operand (& mut self , operand : & mir :: Operand < 'tcx > , location : Location) { self . check_operand_move_size (operand , location) ; } }
};
}
