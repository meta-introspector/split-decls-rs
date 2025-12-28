macro_rules! deps {
    () => {
        IntoBytes!();
    };
}

macro_rules! __size_of {
    () => {
        deps!();
        # [cfg (zerocopy_diagnostic_on_unimplemented_1_78_0)] mod __size_of { # [diagnostic :: on_unimplemented (message = "`{Self}` is unsized" , label = "`IntoBytes` needs all field types to be `Sized` in order to determine whether there is padding" , note = "consider using `#[repr(packed)]` to remove padding" , note = "`IntoBytes` does not require the fields of `#[repr(packed)]` types to be `Sized`")] pub trait Sized : core :: marker :: Sized { } impl < T : core :: marker :: Sized > Sized for T { } # [inline (always)] # [must_use] # [allow (clippy :: needless_maybe_sized)] pub const fn size_of < T : Sized + ? core :: marker :: Sized > () -> usize { core :: mem :: size_of :: < T > () } }
    };
}

__size_of!()