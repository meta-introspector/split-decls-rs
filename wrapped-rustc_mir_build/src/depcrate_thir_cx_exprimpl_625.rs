// Generated macro for impl_625 (impl)
macro_rules! Depcrate_thir_cx_exprimpl_625 {
() => {
// Module: crate::thir::cx::expr
// Provides: {"impl_625"}
// Dependencies: {}
impl ToBorrowKind for AutoBorrowMutability { fn to_borrow_kind (& self) -> BorrowKind { use rustc_middle :: ty :: adjustment :: AllowTwoPhase ; match * self { AutoBorrowMutability :: Mut { allow_two_phase_borrow } => BorrowKind :: Mut { kind : match allow_two_phase_borrow { AllowTwoPhase :: Yes => mir :: MutBorrowKind :: TwoPhaseBorrow , AllowTwoPhase :: No => mir :: MutBorrowKind :: Default , } , } , AutoBorrowMutability :: Not => BorrowKind :: Shared , } } }
};
}
