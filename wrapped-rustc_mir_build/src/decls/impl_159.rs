macro_rules! deps {
    () => {
        CFG!();
        ExitScopes!();
        DropTreeBuilder!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < 'tcx > DropTreeBuilder < 'tcx > for ExitScopes { fn make_block (cfg : & mut CFG < 'tcx >) -> BasicBlock { cfg . start_new_block () } fn link_entry_point (cfg : & mut CFG < 'tcx > , from : BasicBlock , to : BasicBlock) { let term = cfg . block_data_mut (from) . terminator_mut () ; if let TerminatorKind :: UnwindResume = term . kind { term . kind = TerminatorKind :: Goto { target : to } ; } else { span_bug ! (term . source_info . span , "unexpected dummy terminator kind: {:?}" , term . kind) ; } } }
    };
}

impl_159!();