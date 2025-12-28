macro_rules! deps {
    () => {
        TestGraph!();
        Predecessors!();
        Node!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl Predecessors for TestGraph { fn predecessors (& self , node : usize) -> impl Iterator < Item = Self :: Node > { self . predecessors [& node] . iter () . cloned () } }
    };
}

impl_220!()