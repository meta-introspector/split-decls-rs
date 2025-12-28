macro_rules! deps {
    () => {
        MockAnalysis!();
    };
}

macro_rules! mock_body {
    () => {
        deps!();
        # [doc = " Creates a `mir::Body` with a few disconnected basic blocks."] # [doc = ""] # [doc = " This is the `Body` that will be used by the `MockAnalysis` below. The shape of its CFG is not"] # [doc = " important."] fn mock_body < 'tcx > () -> mir :: Body < 'tcx > { let source_info = mir :: SourceInfo :: outermost (DUMMY_SP) ; let mut blocks = IndexVec :: new () ; let mut block = | n , kind | { let nop = mir :: Statement :: new (source_info , mir :: StatementKind :: Nop) ; blocks . push (mir :: BasicBlockData :: new_stmts (std :: iter :: repeat (& nop) . cloned () . take (n) . collect () , Some (mir :: Terminator { source_info , kind }) , false ,)) } ; let dummy_place = mir :: Place { local : mir :: RETURN_PLACE , projection : ty :: List :: empty () } ; block (4 , mir :: TerminatorKind :: Return) ; block (1 , mir :: TerminatorKind :: Return) ; block (2 , mir :: TerminatorKind :: Call { func : mir :: Operand :: Copy (dummy_place . clone ()) , args : [] . into () , destination : dummy_place . clone () , target : Some (mir :: START_BLOCK) , unwind : mir :: UnwindAction :: Continue , call_source : mir :: CallSource :: Misc , fn_span : DUMMY_SP , } ,) ; block (3 , mir :: TerminatorKind :: Return) ; block (0 , mir :: TerminatorKind :: Return) ; block (4 , mir :: TerminatorKind :: Call { func : mir :: Operand :: Copy (dummy_place . clone ()) , args : [] . into () , destination : dummy_place . clone () , target : Some (mir :: START_BLOCK) , unwind : mir :: UnwindAction :: Continue , call_source : mir :: CallSource :: Misc , fn_span : DUMMY_SP , } ,) ; mir :: Body :: new_cfg_only (blocks) }
    };
}

mock_body!()