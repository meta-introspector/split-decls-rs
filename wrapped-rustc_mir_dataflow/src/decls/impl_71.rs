macro_rules! deps {
    () => {
        Analysis!();
        DebugWithContext!();
        Direction!();
        ResultsVisitor!();
        StateDiffCollector!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'tcx , A > ResultsVisitor < 'tcx , A > for StateDiffCollector < A :: Domain > where A : Analysis < 'tcx > , A :: Domain : DebugWithContext < A > , { fn visit_block_start (& mut self , state : & A :: Domain) { if A :: Direction :: IS_FORWARD { self . prev_state . clone_from (state) ; } } fn visit_block_end (& mut self , state : & A :: Domain) { if A :: Direction :: IS_BACKWARD { self . prev_state . clone_from (state) ; } } fn visit_after_early_statement_effect (& mut self , analysis : & mut A , state : & A :: Domain , _statement : & mir :: Statement < 'tcx > , _location : Location ,) { if let Some (before) = self . before . as_mut () { before . push (diff_pretty (state , & self . prev_state , analysis)) ; self . prev_state . clone_from (state) } } fn visit_after_primary_statement_effect (& mut self , analysis : & mut A , state : & A :: Domain , _statement : & mir :: Statement < 'tcx > , _location : Location ,) { self . after . push (diff_pretty (state , & self . prev_state , analysis)) ; self . prev_state . clone_from (state) } fn visit_after_early_terminator_effect (& mut self , analysis : & mut A , state : & A :: Domain , _terminator : & mir :: Terminator < 'tcx > , _location : Location ,) { if let Some (before) = self . before . as_mut () { before . push (diff_pretty (state , & self . prev_state , analysis)) ; self . prev_state . clone_from (state) } } fn visit_after_primary_terminator_effect (& mut self , analysis : & mut A , state : & A :: Domain , _terminator : & mir :: Terminator < 'tcx > , _location : Location ,) { self . after . push (diff_pretty (state , & self . prev_state , analysis)) ; self . prev_state . clone_from (state) } }
    };
}

impl_71!();