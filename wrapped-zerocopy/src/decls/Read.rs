macro_rules! deps {
    () => {
        Exclusive!();
        Aliasing!();
    };
}

macro_rules! Read {
    () => {
        deps!();
        # [doc = " [`Ptr`](crate::Ptr) referents that permit unsynchronized read operations."] # [doc = ""] # [doc = " `T: Read<A, R>` implies that a pointer to `T` with aliasing `A` permits"] # [doc = " unsynchronized read operations. This can be because `A` is [`Exclusive`] or"] # [doc = " because `T` does not permit interior mutation."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `T: Read<A, R>` if either of the following conditions holds:"] # [doc = " - `A` is [`Exclusive`]"] # [doc = " - `T` implements [`Immutable`](crate::Immutable)"] # [doc = ""] # [doc = " As a consequence, if `T: Read<A, R>`, then any `Ptr<T, (A, ...)>` is"] # [doc = " permitted to perform unsynchronized reads from its referent."] pub trait Read < A : Aliasing , R > { }
    };
}

Read!();