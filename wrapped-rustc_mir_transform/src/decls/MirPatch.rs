macro_rules! MirPatch {
    () => {
        # [doc = " This struct lets you \"patch\" a MIR body, i.e. modify it. You can queue up"] # [doc = " various changes, such as the addition of new statements and basic blocks"] # [doc = " and replacement of terminators, and then apply the queued changes all at"] # [doc = " once with `apply`. This is useful for MIR transformation passes."] pub (crate) struct MirPatch < 'tcx > { term_patch_map : IndexVec < BasicBlock , Option < TerminatorKind < 'tcx > > > , new_blocks : Vec < BasicBlockData < 'tcx > > , new_statements : Vec < (Location , StatementKind < 'tcx >) > , new_locals : Vec < LocalDecl < 'tcx > > , resume_block : Option < BasicBlock > , unreachable_cleanup_block : Option < BasicBlock > , unreachable_no_cleanup_block : Option < BasicBlock > , terminate_block : Option < (BasicBlock , UnwindTerminateReason) > , body_span : Span , next_local : usize , }
    };
}

MirPatch!()