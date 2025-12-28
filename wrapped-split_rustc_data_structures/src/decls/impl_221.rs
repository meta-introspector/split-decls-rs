macro_rules! deps {
    () => {
        Successors!();
        TestGraph!();
        Node!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl Successors for TestGraph { fn successors (& self , node : usize) -> impl Iterator < Item = Self :: Node > { self . successors [& node] . iter () . cloned () } }
    };
}

impl_221!()