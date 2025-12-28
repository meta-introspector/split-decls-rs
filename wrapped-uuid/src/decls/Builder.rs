macro_rules! deps {
    () => {
        Uuid!();
        Variant!();
        Version!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [doc = " A builder for creating a UUID."] # [doc = ""] # [doc = " This type is useful if you need to mutate individual fields of a [`Uuid`]"] # [doc = " while constructing it. Since the [`Uuid`] type is `Copy`, it doesn't offer"] # [doc = " any methods to mutate in place. They live on the `Builder` instead."] # [doc = ""] # [doc = " The `Builder` type also always exposes APIs to construct [`Uuid`]s for any"] # [doc = " version without needing crate features or additional dependencies. It's a"] # [doc = " lower-level API than the methods on [`Uuid`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Creating a version 4 UUID from externally generated random bytes:"] # [doc = ""] # [doc = " ```"] # [doc = " # use uuid::{Builder, Version, Variant};"] # [doc = " # let rng = || ["] # [doc = " #     70, 235, 208, 238, 14, 109, 67, 201, 185, 13, 204, 195, 90,"] # [doc = " # 145, 63, 62,"] # [doc = " # ];"] # [doc = " let random_bytes = rng();"] # [doc = ""] # [doc = " let uuid = Builder::from_random_bytes(random_bytes).into_uuid();"] # [doc = ""] # [doc = " assert_eq!(Some(Version::Random), uuid.get_version());"] # [doc = " assert_eq!(Variant::RFC4122, uuid.get_variant());"] # [doc = " ```"] # [allow (missing_copy_implementations)] # [derive (Debug)] pub struct Builder (Uuid) ;
    };
}

Builder!()