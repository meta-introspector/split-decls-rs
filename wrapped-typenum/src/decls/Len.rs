macro_rules! deps {
    () => {
        UInt!();
        Unsigned!();
    };
}

macro_rules! Len {
    () => {
        deps!();
        # [doc = " A **type operator** that gives the length of an `Array` or the number of bits in a `UInt`."] # [allow (clippy :: len_without_is_empty)] pub trait Len { # [doc = " The length as a type-level unsigned integer."] type Output : crate :: Unsigned ; # [doc = " This function isn't used in this crate, but may be useful for others."] fn len (& self) -> Self :: Output ; }
    };
}

Len!()