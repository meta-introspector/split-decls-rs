macro_rules! deps {
    () => {
        Punct!();
        Literal!();
        Ident!();
    };
}

macro_rules! Leaf {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum Leaf < S > { Literal (Literal < S >) , Punct (Punct < S >) , Ident (Ident < S >) , }
    };
}

Leaf!()