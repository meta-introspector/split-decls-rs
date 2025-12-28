macro_rules! deps {
    () => {
        KnownLayout!();
    };
}

macro_rules! MaybeUninit {
    () => {
        deps!();
        # [doc = " A wrapper type to construct uninitialized instances of `T`."] # [doc = ""] # [doc = " `MaybeUninit` is identical to the [standard library"] # [doc = " `MaybeUninit`][core-maybe-uninit] type except that it supports unsized"] # [doc = " types."] # [doc = ""] # [doc = " # Layout"] # [doc = ""] # [doc = " The same layout guarantees and caveats apply to `MaybeUninit<T>` as apply to"] # [doc = " the [standard library `MaybeUninit`][core-maybe-uninit] with one exception:"] # [doc = " for `T: !Sized`, there is no single value for `T`'s size. Instead, for such"] # [doc = " types, the following are guaranteed:"] # [doc = " - Every [valid size][valid-size] for `T` is a valid size for"] # [doc = "   `MaybeUninit<T>` and vice versa"] # [doc = " - Given `t: *const T` and `m: *const MaybeUninit<T>` with identical fat"] # [doc = "   pointer metadata, `t` and `m` address the same number of bytes (and"] # [doc = "   likewise for `*mut`)"] # [doc = ""] # [doc = " [core-maybe-uninit]: core::mem::MaybeUninit"] # [doc = " [valid-size]: crate::KnownLayout#what-is-a-valid-size"] # [repr (transparent)] # [doc (hidden)] pub struct MaybeUninit < T : ? Sized + KnownLayout > (T :: MaybeUninit ,) ;
    };
}

MaybeUninit!();