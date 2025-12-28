macro_rules! deps {
    () => {
        Visitor!();
    };
}

macro_rules! Visitable {
    () => {
        deps!();
        pub trait Visitable { fn visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { self . super_visit (visitor) } fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > ; }
    };
}

Visitable!()