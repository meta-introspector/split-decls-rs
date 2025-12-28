macro_rules! Lookup {
    () => {
        # [doc = " The `Lookup` trait is a more flexible variant on [`std::borrow::Borrow`]"] # [doc = " and [`std::borrow::ToOwned`]."] # [doc = ""] # [doc = " It is implemented by \"some type that can be used as the lookup key for `O`\"."] # [doc = " This means that `self` can be hashed and compared for equality with values"] # [doc = " of type `O` without actually creating an owned value. It `self` needs to be interned,"] # [doc = " it can be converted into an equivalent value of type `O`."] # [doc = ""] # [doc = " The canonical example is `&str: Lookup<String>`. However, this example"] # [doc = " alone can be handled by [`std::borrow::Borrow`][]. In our case, we may have"] # [doc = " multiple keys accumulated into a struct, like `ViewStruct: Lookup<(K1, ...)>`,"] # [doc = " where `struct ViewStruct<L1: Lookup<K1>...>(K1...)`. The `Borrow` trait"] # [doc = " requires that `&(K1...)` be convertible to `&ViewStruct` which just isn't"] # [doc = " possible. `Lookup` instead offers direct `hash` and `eq` methods."] pub trait Lookup < O > { fn into_owned (self) -> O ; }
    };
}

Lookup!()