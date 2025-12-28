macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! Simple {
    () => {
        deps!();
        # [doc = " Format a [`Uuid`] as a simple string, like"] # [doc = " `67e5504410b1426f9247bb680e5fe0c8`."] # [derive (Clone , Copy , Debug , Default , Eq , Hash , Ord , PartialEq , PartialOrd)] # [cfg_attr (all (uuid_unstable , feature = "zerocopy") , derive (zerocopy :: IntoBytes , zerocopy :: FromBytes , zerocopy :: KnownLayout , zerocopy :: Immutable , zerocopy :: Unaligned))] # [repr (transparent)] pub struct Simple (Uuid) ;
    };
}

Simple!();