macro_rules! JoinedArgs {
    () => {
        # [derive (Clone , Default , Debug , PartialEq , Eq)] # [cfg_attr (feature = "schema" , derive (schemars :: JsonSchema))] pub (crate) struct JoinedArgs { inner : Vec < String > , }
    };
}

JoinedArgs!()