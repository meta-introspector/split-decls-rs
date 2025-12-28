macro_rules! deps {
    () => {
        ComesFromAllowExpect!();
    };
}

macro_rules! MarkSymbolVisitor {
    () => {
        deps!();
        struct MarkSymbolVisitor < 'tcx > { worklist : Vec < (LocalDefId , ComesFromAllowExpect) > , tcx : TyCtxt < 'tcx > , maybe_typeck_results : Option < & 'tcx ty :: TypeckResults < 'tcx > > , scanned : LocalDefIdSet , live_symbols : LocalDefIdSet , repr_unconditionally_treats_fields_as_live : bool , repr_has_repr_simd : bool , in_pat : bool , ignore_variant_stack : Vec < DefId > , ignored_derived_traits : LocalDefIdMap < FxIndexSet < DefId > > , }
    };
}

MarkSymbolVisitor!()