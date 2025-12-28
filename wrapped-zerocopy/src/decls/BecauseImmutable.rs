macro_rules! BecauseImmutable {
    () => {
        # [doc = " Unsynchronized reads are permitted because no live [`Ptr`](crate::Ptr)s or"] # [doc = " references permit interior mutation."] # [derive (Copy , Clone , Debug)] # [doc (hidden)] pub enum BecauseImmutable { }
    };
}

BecauseImmutable!()