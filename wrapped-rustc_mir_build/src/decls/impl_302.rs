macro_rules! deps {
    () => {
        ToBorrowKind!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl ToBorrowKind for AutoBorrowMutability { fn to_borrow_kind (& self) -> BorrowKind { use rustc_middle :: ty :: adjustment :: AllowTwoPhase ; match * self { AutoBorrowMutability :: Mut { allow_two_phase_borrow } => BorrowKind :: Mut { kind : match allow_two_phase_borrow { AllowTwoPhase :: Yes => mir :: MutBorrowKind :: TwoPhaseBorrow , AllowTwoPhase :: No => mir :: MutBorrowKind :: Default , } , } , AutoBorrowMutability :: Not => BorrowKind :: Shared , } } }
    };
}

impl_302!();