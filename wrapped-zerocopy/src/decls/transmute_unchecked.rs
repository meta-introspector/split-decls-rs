macro_rules! transmute_unchecked {
    () => {
        # [doc = " Unsafely transmutes the given `src` into a type `Dst`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The value `src` must be a valid instance of `Dst`."] # [inline (always)] pub (crate) const unsafe fn transmute_unchecked < Src , Dst > (src : Src) -> Dst { static_assert ! (Src , Dst => core :: mem :: size_of ::< Src > () == core :: mem :: size_of ::< Dst > ()) ; # [repr (C)] union Transmute < Src , Dst > { src : ManuallyDrop < Src > , dst : ManuallyDrop < Dst > , } unsafe { ManuallyDrop :: into_inner (Transmute { src : ManuallyDrop :: new (src) } . dst) } }
    };
}

transmute_unchecked!();