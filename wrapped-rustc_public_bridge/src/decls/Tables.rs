macro_rules! deps {
    () => {
        Bridge!();
        IndexMap!();
    };
}

macro_rules! Tables {
    () => {
        deps!();
        pub struct Tables < 'tcx , B : Bridge > { pub def_ids : IndexMap < DefId , B :: DefId > , pub alloc_ids : IndexMap < AllocId , B :: AllocId > , pub spans : IndexMap < rustc_span :: Span , B :: Span > , pub types : IndexMap < Ty < 'tcx > , B :: Ty > , pub instances : IndexMap < ty :: Instance < 'tcx > , B :: InstanceDef > , pub ty_consts : IndexMap < ty :: Const < 'tcx > , B :: TyConstId > , pub mir_consts : IndexMap < mir :: Const < 'tcx > , B :: MirConstId > , pub layouts : IndexMap < rustc_abi :: Layout < 'tcx > , B :: Layout > , }
    };
}

Tables!();