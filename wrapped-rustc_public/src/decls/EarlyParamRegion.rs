macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! EarlyParamRegion {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct EarlyParamRegion { pub index : u32 , pub name : Symbol , }
    };
}

EarlyParamRegion!();