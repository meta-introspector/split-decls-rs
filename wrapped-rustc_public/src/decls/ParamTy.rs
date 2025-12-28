macro_rules! ParamTy {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct ParamTy { pub index : u32 , pub name : String , }
    };
}

ParamTy!()