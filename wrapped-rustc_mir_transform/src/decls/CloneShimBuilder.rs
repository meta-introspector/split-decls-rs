macro_rules! CloneShimBuilder {
    () => {
        struct CloneShimBuilder < 'tcx > { tcx : TyCtxt < 'tcx > , def_id : DefId , local_decls : IndexVec < Local , LocalDecl < 'tcx > > , blocks : IndexVec < BasicBlock , BasicBlockData < 'tcx > > , span : Span , sig : ty :: FnSig < 'tcx > , }
    };
}

CloneShimBuilder!();