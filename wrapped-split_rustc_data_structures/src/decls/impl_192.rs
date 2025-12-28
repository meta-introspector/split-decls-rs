macro_rules! deps {
    () => {
        NumEdges!();
        Sccs!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < N : Idx , S : Idx + Ord > NumEdges for Sccs < N , S > { fn num_edges (& self) -> usize { self . scc_data . all_successors . len () } }
    };
}

impl_192!();