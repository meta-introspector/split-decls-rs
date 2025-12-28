macro_rules! deps {
    () => {
        CollectionMode!();
    };
}

macro_rules! OpaqueTypeCollector {
    () => {
        deps!();
        struct OpaqueTypeCollector < 'tcx > { tcx : TyCtxt < 'tcx > , opaques : Vec < LocalDefId > , # [doc = " The `DefId` of the item which we are collecting opaque types for."] item : LocalDefId , # [doc = " Avoid infinite recursion due to recursive declarations."] seen : FxHashSet < LocalDefId > , span : Option < Span > , mode : CollectionMode , }
    };
}

OpaqueTypeCollector!()