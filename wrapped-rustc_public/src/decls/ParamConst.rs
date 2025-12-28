macro_rules! ParamConst {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct ParamConst { pub index : u32 , pub name : String , }
    };
}

ParamConst!();