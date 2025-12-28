macro_rules! Interned {
    () => {
        # [doc = " A reference to a value that is interned, and is known to be unique."] # [doc = ""] # [doc = " Note that it is possible to have a `T` and a `Interned<T>` that are (or"] # [doc = " refer to) equal but different values. But if you have two different"] # [doc = " `Interned<T>`s, they both refer to the same value, at a single location in"] # [doc = " memory. This means that equality and hashing can be done on the value's"] # [doc = " address rather than the value's contents, which can improve performance."] # [doc = ""] # [doc = " The `PrivateZst` field means you can pattern match with `Interned(v, _)`"] # [doc = " but you can only construct a `Interned` with `new_unchecked`, and not"] # [doc = " directly."] # [rustc_pass_by_value] pub struct Interned < 'a , T > (pub & 'a T , pub private :: PrivateZst) ;
    };
}

Interned!();