macro_rules! deps {
    () => {
        Spacing!();
    };
}

macro_rules! Punct {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Punct < S > { pub char : char , pub spacing : Spacing , pub span : S , }
    };
}

Punct!()