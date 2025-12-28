macro_rules! deps {
    () => {
        Name!();
        ConstValue!();
    };
}

macro_rules! Variables {
    () => {
        deps!();
        # [doc = " Variables of a query."] # [derive (Debug , Clone , Default , Serialize , Eq , PartialEq)] # [serde (transparent)] pub struct Variables (BTreeMap < Name , ConstValue >) ;
    };
}

Variables!();