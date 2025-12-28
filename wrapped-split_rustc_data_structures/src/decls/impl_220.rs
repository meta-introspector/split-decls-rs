macro_rules! deps {
    () => {
        Predecessors!();
        Node!();
        TestGraph!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl Predecessors for TestGraph { fn predecessors (& self , node : usize) -> impl Iterator < Item = Self :: Node > { self . predecessors [& node] . iter () . cloned () } }
    };
}

impl_220!();