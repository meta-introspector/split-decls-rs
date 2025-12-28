macro_rules! deps {
    () => {
        Sccs!();
        Successors!();
        Node!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < N : Idx , S : Idx + Ord > Successors for Sccs < N , S > { fn successors (& self , node : S) -> impl Iterator < Item = Self :: Node > { self . successors (node) . iter () . cloned () } }
    };
}

impl_193!();