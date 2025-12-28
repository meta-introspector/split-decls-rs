macro_rules! deps {
    () => {
        QueryRevisions!();
        QueryOriginRef!();
        QueryRevisionsExtra!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl QueryRevisions { # [cfg (feature = "salsa_unstable")] pub (crate) fn allocation_size (& self) -> usize { let QueryRevisions { changed_at : _ , durability : _ , verified_final : _ , origin , extra , # [cfg (feature = "accumulator")] accumulated_inputs : _ , } = self ; let mut memory = 0 ; if let QueryOriginRef :: Derived (query_edges) | QueryOriginRef :: DerivedUntracked (query_edges) = origin . as_ref () { memory += std :: mem :: size_of_val (query_edges) ; } if let Some (extra) = extra . 0 . as_ref () { memory += std :: mem :: size_of :: < QueryRevisionsExtra > () ; memory += extra . allocation_size () ; } memory } }
    };
}

impl_460!();