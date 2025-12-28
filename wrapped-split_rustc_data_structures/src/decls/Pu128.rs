macro_rules! Pu128 {
    () => {
        # [doc = " A packed 128-bit integer. Useful for reducing the size of structures in"] # [doc = " some cases."] # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq , PartialOrd , Ord)] # [repr (packed (8))] pub struct Pu128 (pub u128) ;
    };
}

Pu128!();