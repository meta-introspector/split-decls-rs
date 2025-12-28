macro_rules! deps {
    () => {
        BinderLevel!();
    };
}

macro_rules! V0SymbolMangler {
    () => {
        deps!();
        struct V0SymbolMangler < 'tcx > { tcx : TyCtxt < 'tcx > , binders : Vec < BinderLevel > , out : String , is_exportable : bool , # [doc = " The length of the prefix in `out` (e.g. 2 for `_R`)."] start_offset : usize , # [doc = " The values are start positions in `out`, in bytes."] paths : FxHashMap < (DefId , & 'tcx [GenericArg < 'tcx >]) , usize > , types : FxHashMap < Ty < 'tcx > , usize > , consts : FxHashMap < ty :: Const < 'tcx > , usize > , }
    };
}

V0SymbolMangler!();