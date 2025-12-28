macro_rules! deps {
    () => {
        NumEdges!();
        VecGraph!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < N : Idx , const BR : bool > NumEdges for VecGraph < N , BR > { fn num_edges (& self) -> usize { match BR { false => self . edge_targets . len () , true => self . edge_targets . len () / 2 , } } }
    };
}

impl_212!()