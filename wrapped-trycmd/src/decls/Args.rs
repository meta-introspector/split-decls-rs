macro_rules! deps {
    () => {
        JoinedArgs!();
    };
}

macro_rules! Args {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq , serde :: Deserialize , serde :: Serialize)] # [cfg_attr (feature = "schema" , derive (schemars :: JsonSchema))] # [serde (untagged)] pub (crate) enum Args { Joined (JoinedArgs) , Split (Vec < String >) , }
    };
}

Args!();