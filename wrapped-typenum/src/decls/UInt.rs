macro_rules! deps {
    () => {
        Bit!();
        B1!();
        Eq!();
        UTerm!();
        Unsigned!();
        Ord!();
        B0!();
    };
}

macro_rules! UInt {
    () => {
        deps!();
        # [doc = " `UInt` is defined recursively, where `B` is the least significant bit and `U` is the rest"] # [doc = " of the number. Conceptually, `U` should be bound by the trait `Unsigned` and `B` should"] # [doc = " be bound by the trait `Bit`, but enforcing these bounds causes linear instead of"] # [doc = " logrithmic scaling in some places, so they are left off for now. They may be enforced in"] # [doc = " future."] # [doc = ""] # [doc = " In order to keep numbers unique, leading zeros are not allowed, so `UInt<UTerm, B0>` is"] # [doc = " forbidden."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use typenum::{UInt, UTerm, B0, B1};"] # [doc = ""] # [doc = " # #[allow(dead_code)]"] # [doc = " type U6 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>;"] # [doc = " ```"] # [derive (Eq , PartialEq , Ord , PartialOrd , Clone , Copy , Hash , Debug , Default)] # [cfg_attr (feature = "scale_info" , derive (scale_info :: TypeInfo))] pub struct UInt < U , B > { # [doc = " The more significant bits of `Self`."] pub (crate) msb : U , # [doc = " The least significant bit of `Self`."] pub (crate) lsb : B , }
    };
}

UInt!()