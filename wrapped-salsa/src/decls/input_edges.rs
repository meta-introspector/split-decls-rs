macro_rules! deps {
    () => {
        QueryEdge!();
        DatabaseKeyIndex!();
        QueryEdgeKind!();
    };
}

macro_rules! input_edges {
    () => {
        deps!();
        # [doc = " Returns the (tracked) inputs that were executed in computing this memoized value."] # [doc = ""] # [doc = " These will always be in execution order."] # [cfg (feature = "accumulator")] pub (crate) fn input_edges (input_outputs : & [QueryEdge] ,) -> impl DoubleEndedIterator < Item = DatabaseKeyIndex > + use < '_ > { input_outputs . iter () . filter_map (| & edge | match edge . kind () { QueryEdgeKind :: Input (dependency_index) => Some (dependency_index) , QueryEdgeKind :: Output (_) => None , }) }
    };
}

input_edges!();