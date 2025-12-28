macro_rules! MoveCheckVisitor {
    () => {
        struct MoveCheckVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , body : & 'tcx mir :: Body < 'tcx > , # [doc = " Spans for move size lints already emitted. Helps avoid duplicate lints."] move_size_spans : Vec < Span > , }
    };
}

MoveCheckVisitor!()