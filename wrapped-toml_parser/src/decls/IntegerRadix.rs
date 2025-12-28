macro_rules! IntegerRadix {
    () => {
        # [derive (Copy , Clone , Default , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] pub enum IntegerRadix { # [default] Dec , Hex , Oct , Bin , }
    };
}

IntegerRadix!();