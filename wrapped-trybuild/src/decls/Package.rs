macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! Package {
    () => {
        deps!();
        # [derive (Serialize , Debug)] pub (crate) struct Package { pub name : String , pub version : String , pub edition : Edition , # [serde (skip_serializing_if = "Option::is_none")] pub resolver : Option < String > , pub publish : bool , }
    };
}

Package!()