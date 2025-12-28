macro_rules! deps {
    () => {
        Node!();
        VecGraph!();
        Successors!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl < N : Idx + Ord , const BR : bool > Successors for VecGraph < N , BR > { fn successors (& self , node : N) -> impl Iterator < Item = Self :: Node > { self . successors (node) . iter () . cloned () } }
    };
}

impl_213!();