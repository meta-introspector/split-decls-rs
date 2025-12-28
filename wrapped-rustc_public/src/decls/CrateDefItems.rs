macro_rules! deps {
    () => {
        CrateDef!();
        AssocItems!();
    };
}

macro_rules! CrateDefItems {
    () => {
        deps!();
        # [doc = " A trait for retrieving all items from a definition within a crate."] pub trait CrateDefItems : CrateDef { # [doc = " Retrieve all associated items from a definition."] fn associated_items (& self) -> AssocItems { with (| cx | cx . associated_items (self . def_id ())) } }
    };
}

CrateDefItems!()