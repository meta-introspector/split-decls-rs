macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! NonNilUuid {
    () => {
        deps!();
        # [doc = " A UUID that is guaranteed not to be the [nil UUID](https://www.ietf.org/rfc/rfc9562.html#name-nil-uuid)."] # [doc = ""] # [doc = " This is useful for representing optional UUIDs more efficiently, as `Option<NonNilUuid>`"] # [doc = " takes up the same space as `Uuid`."] # [doc = ""] # [doc = " Note that `Uuid`s created by the following methods are guaranteed to be non-nil:"] # [doc = ""] # [doc = " - [`Uuid::new_v1`]"] # [doc = " - [`Uuid::now_v1`]"] # [doc = " - [`Uuid::new_v3`]"] # [doc = " - [`Uuid::new_v4`]"] # [doc = " - [`Uuid::new_v5`]"] # [doc = " - [`Uuid::new_v6`]"] # [doc = " - [`Uuid::now_v6`]"] # [doc = " - [`Uuid::new_v7`]"] # [doc = " - [`Uuid::now_v7`]"] # [doc = " - [`Uuid::new_v8`]"] # [doc = ""] # [doc = " # ABI"] # [doc = ""] # [doc = " The `NonNilUuid` type does not yet have a stable ABI. Its representation or alignment"] # [doc = " may change. It is currently only guaranteed that `NonNilUuid` and `Option<NonNilUuid>`"] # [doc = " are the same size as `Uuid`."] # [repr (transparent)] # [derive (Copy , Clone , PartialEq , Eq , Hash)] pub struct NonNilUuid (NonZeroU128) ;
    };
}

NonNilUuid!()