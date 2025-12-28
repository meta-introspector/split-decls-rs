macro_rules! deps {
    () => {
        Symbol!();
        CrateNum!();
    };
}

macro_rules! Crate {
    () => {
        deps!();
        # [doc = " Holds information about a crate."] # [derive (Clone , PartialEq , Eq , Debug , Serialize)] pub struct Crate { pub id : CrateNum , pub name : Symbol , pub is_local : bool , }
    };
}

Crate!()