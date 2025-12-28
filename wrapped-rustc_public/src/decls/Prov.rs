macro_rules! Prov {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub struct Prov (pub AllocId) ;
    };
}

Prov!();