macro_rules! deps {
    () => {
        QueryEdge!();
        QueryOriginRef!();
        DatabaseKeyIndex!();
    };
}

macro_rules! impl_470 {
    () => {
        deps!();
        impl < 'a > QueryOriginRef < 'a > { # [doc = " Indices for queries *read* by this query"] # [inline] # [cfg (feature = "accumulator")] pub (crate) fn inputs (self) -> impl DoubleEndedIterator < Item = DatabaseKeyIndex > + use < 'a > { let opt_edges = match self { QueryOriginRef :: Derived (edges) | QueryOriginRef :: DerivedUntracked (edges) => Some (edges) , QueryOriginRef :: Assigned (_) | QueryOriginRef :: FixpointInitial => None , } ; opt_edges . into_iter () . flat_map (input_edges) } # [doc = " Indices for queries *written* by this query (if any)"] pub (crate) fn outputs (self) -> impl DoubleEndedIterator < Item = DatabaseKeyIndex > + use < 'a > { let opt_edges = match self { QueryOriginRef :: Derived (edges) | QueryOriginRef :: DerivedUntracked (edges) => Some (edges) , QueryOriginRef :: Assigned (_) | QueryOriginRef :: FixpointInitial => None , } ; opt_edges . into_iter () . flat_map (output_edges) } # [inline] pub (crate) fn edges (self) -> & 'a [QueryEdge] { let opt_edges = match self { QueryOriginRef :: Derived (edges) | QueryOriginRef :: DerivedUntracked (edges) => Some (edges) , QueryOriginRef :: Assigned (_) | QueryOriginRef :: FixpointInitial => None , } ; opt_edges . unwrap_or_default () } }
    };
}

impl_470!();