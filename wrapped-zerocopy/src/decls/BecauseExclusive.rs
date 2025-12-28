macro_rules! BecauseExclusive {
    () => {
        # [doc = " Unsynchronized reads are permitted because only one live [`Ptr`](crate::Ptr)"] # [doc = " or reference may exist to the referent bytes at a time."] # [derive (Copy , Clone , Debug)] # [doc (hidden)] pub enum BecauseExclusive { }
    };
}

BecauseExclusive!()