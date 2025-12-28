macro_rules! deps {
    () => {
        Literal!();
        Punct!();
        Ident!();
    };
}

macro_rules! Leaf {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum Leaf < S > { Literal (Literal < S >) , Punct (Punct < S >) , Ident (Ident < S >) , }
    };
}

Leaf!();