macro_rules! AddBounds {
    () => {
        # [doc = " Changes how bounds are added"] # [allow (clippy :: manual_non_exhaustive)] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum AddBounds { # [doc = " Add for fields and generics"] Both , # [doc = " Fields only"] Fields , # [doc = " Generics only"] Generics , # [doc = " None"] None , # [doc (hidden)] __Nonexhaustive , }
    };
}

AddBounds!()