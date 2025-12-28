macro_rules! deps {
    () => {
        DelimiterKind!();
    };
}

macro_rules! Delimiter {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct Delimiter < S > { pub open : S , pub close : S , pub kind : DelimiterKind , }
    };
}

Delimiter!();