macro_rules! deps {
    () => {
        RuleKind!();
    };
}

macro_rules! Rule {
    () => {
        deps!();
        # [derive (Debug , Deserialize)] pub struct Rule { pub kind : RuleKind , pub trait_name : Vec < String > , # [serde (default)] pub apply_to : Vec < String > , pub condition : String , }
    };
}

Rule!();