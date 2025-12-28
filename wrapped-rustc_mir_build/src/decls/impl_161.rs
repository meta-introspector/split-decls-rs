macro_rules! deps {
    () => {
        CFG!();
        CoroutineDrop!();
        DropTreeBuilder!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < 'tcx > DropTreeBuilder < 'tcx > for CoroutineDrop { fn make_block (cfg : & mut CFG < 'tcx >) -> BasicBlock { cfg . start_new_block () } fn link_entry_point (cfg : & mut CFG < 'tcx > , from : BasicBlock , to : BasicBlock) { let term = cfg . block_data_mut (from) . terminator_mut () ; if let TerminatorKind :: Yield { ref mut drop , .. } = term . kind { * drop = Some (to) ; } else if let TerminatorKind :: Drop { ref mut drop , .. } = term . kind { * drop = Some (to) ; } else { span_bug ! (term . source_info . span , "cannot enter coroutine drop tree from {:?}" , term . kind) } } }
    };
}

impl_161!();