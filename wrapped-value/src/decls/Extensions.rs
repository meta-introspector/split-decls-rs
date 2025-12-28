macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! Extensions {
    () => {
        deps!();
        # [doc = " Extensions of a query."] # [derive (Debug , Clone , Default , Serialize , Eq , PartialEq)] # [serde (transparent)] pub struct Extensions (pub HashMap < String , crate :: ConstValue >) ;
    };
}

Extensions!()