macro_rules! deps {
    () => {
        Delimiter!();
    };
}

macro_rules! Subtree {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Subtree < S > { pub delimiter : Delimiter < S > , # [doc = " Number of following token trees that belong to this subtree, excluding this subtree."] pub len : u32 , }
    };
}

Subtree!();