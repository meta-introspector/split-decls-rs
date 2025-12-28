macro_rules! deps {
    () => {
        ResultsCursor!();
        Analysis!();
    };
}

macro_rules! ResultsVisitor {
    () => {
        deps!();
        # [doc = " A visitor over the results of an `Analysis`. Use this when you want to inspect domain values in"] # [doc = " many or all locations; use `ResultsCursor` if you want to inspect domain values only in certain"] # [doc = " locations."] pub trait ResultsVisitor < 'tcx , A > where A : Analysis < 'tcx > , { fn visit_block_start (& mut self , _state : & A :: Domain) { } # [doc = " Called after the \"early\" effect of the given statement is applied to `state`."] fn visit_after_early_statement_effect (& mut self , _analysis : & mut A , _state : & A :: Domain , _statement : & mir :: Statement < 'tcx > , _location : Location ,) { } # [doc = " Called after the \"primary\" effect of the given statement is applied to `state`."] fn visit_after_primary_statement_effect (& mut self , _analysis : & mut A , _state : & A :: Domain , _statement : & mir :: Statement < 'tcx > , _location : Location ,) { } # [doc = " Called after the \"early\" effect of the given terminator is applied to `state`."] fn visit_after_early_terminator_effect (& mut self , _analysis : & mut A , _state : & A :: Domain , _terminator : & mir :: Terminator < 'tcx > , _location : Location ,) { } # [doc = " Called after the \"primary\" effect of the given terminator is applied to `state`."] # [doc = ""] # [doc = " The `call_return_effect` (if one exists) will *not* be applied to `state`."] fn visit_after_primary_terminator_effect (& mut self , _analysis : & mut A , _state : & A :: Domain , _terminator : & mir :: Terminator < 'tcx > , _location : Location ,) { } fn visit_block_end (& mut self , _state : & A :: Domain) { } }
    };
}

ResultsVisitor!();