macro_rules! deps {
    () => {
        QueryCtxt!();
    };
}

macro_rules! force_from_dep_node {
    () => {
        deps!();
        fn force_from_dep_node < 'tcx , Q > (query : Q , tcx : TyCtxt < 'tcx > , dep_node : DepNode) -> bool where Q : QueryConfig < QueryCtxt < 'tcx > > , { debug_assert ! (dep_node . kind != dep_kinds :: codegen_unit , "calling force_from_dep_node() on dep_kinds::codegen_unit") ; if let Some (key) = Q :: Key :: recover (tcx , & dep_node) { force_query (query , QueryCtxt :: new (tcx) , key , dep_node) ; true } else { false } }
    };
}

force_from_dep_node!();