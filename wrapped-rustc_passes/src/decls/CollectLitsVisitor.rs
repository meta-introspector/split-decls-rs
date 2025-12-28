macro_rules! CollectLitsVisitor {
    () => {
        struct CollectLitsVisitor < 'tcx > { lit_exprs : Vec < & 'tcx hir :: Expr < 'tcx > > , }
    };
}

CollectLitsVisitor!()