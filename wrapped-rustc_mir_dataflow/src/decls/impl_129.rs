macro_rules! deps {
    () => {
        GenKill!();
        TransferFunction!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < 'tcx , T > Visitor < 'tcx > for TransferFunction < '_ , T > where T : GenKill < Local > , { fn visit_statement (& mut self , stmt : & Statement < 'tcx > , location : Location) { self . super_statement (stmt , location) ; if let StatementKind :: StorageDead (local) = stmt . kind { self . trans . kill (local) ; } } fn visit_rvalue (& mut self , rvalue : & Rvalue < 'tcx > , location : Location) { self . super_rvalue (rvalue , location) ; match rvalue { Rvalue :: RawPtr (_ , borrowed_place) | Rvalue :: Ref (_ , BorrowKind :: Mut { .. } | BorrowKind :: Shared , borrowed_place) => { if ! borrowed_place . is_indirect () { self . trans . gen_ (borrowed_place . local) ; } } Rvalue :: Cast (..) | Rvalue :: Ref (_ , BorrowKind :: Fake (_) , _) | Rvalue :: ShallowInitBox (..) | Rvalue :: Use (..) | Rvalue :: ThreadLocalRef (..) | Rvalue :: Repeat (..) | Rvalue :: Len (..) | Rvalue :: BinaryOp (..) | Rvalue :: NullaryOp (..) | Rvalue :: UnaryOp (..) | Rvalue :: Discriminant (..) | Rvalue :: Aggregate (..) | Rvalue :: CopyForDeref (..) | Rvalue :: WrapUnsafeBinder (..) => { } } } fn visit_terminator (& mut self , terminator : & Terminator < 'tcx > , location : Location) { self . super_terminator (terminator , location) ; match terminator . kind { TerminatorKind :: Drop { place : dropped_place , .. } => { if ! dropped_place . is_indirect () { self . trans . gen_ (dropped_place . local) ; } } TerminatorKind :: UnwindTerminate (_) | TerminatorKind :: Assert { .. } | TerminatorKind :: Call { .. } | TerminatorKind :: FalseEdge { .. } | TerminatorKind :: FalseUnwind { .. } | TerminatorKind :: CoroutineDrop | TerminatorKind :: Goto { .. } | TerminatorKind :: InlineAsm { .. } | TerminatorKind :: UnwindResume | TerminatorKind :: Return | TerminatorKind :: TailCall { .. } | TerminatorKind :: SwitchInt { .. } | TerminatorKind :: Unreachable | TerminatorKind :: Yield { .. } => { } } } }
    };
}

impl_129!()