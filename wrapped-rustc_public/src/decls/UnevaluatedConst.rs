macro_rules! deps {
    () => {
        Promoted!();
        GenericArgs!();
    };
}

macro_rules! UnevaluatedConst {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct UnevaluatedConst { pub def : ConstDef , pub args : GenericArgs , pub promoted : Option < Promoted > , }
    };
}

UnevaluatedConst!()