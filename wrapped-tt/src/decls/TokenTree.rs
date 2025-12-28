macro_rules! deps {
    () => {
        Subtree!();
        Leaf!();
    };
}

macro_rules! TokenTree {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum TokenTree < S = u32 > { Leaf (Leaf < S >) , Subtree (Subtree < S >) , }
    };
}

TokenTree!()