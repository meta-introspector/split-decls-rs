macro_rules! deps {
    () => {
        DatabaseKeyIndex!();
        QueryEdgeKind!();
        QueryEdge!();
    };
}

macro_rules! output_edges {
    () => {
        deps!();
        # [doc = " Returns the (tracked) outputs that were executed in computing this memoized value."] # [doc = ""] # [doc = " These will always be in execution order."] pub (crate) fn output_edges (input_outputs : & [QueryEdge] ,) -> impl DoubleEndedIterator < Item = DatabaseKeyIndex > + use < '_ > { input_outputs . iter () . filter_map (| & edge | match edge . kind () { QueryEdgeKind :: Output (dependency_index) => Some (dependency_index) , QueryEdgeKind :: Input (_) => None , }) }
    };
}

output_edges!()