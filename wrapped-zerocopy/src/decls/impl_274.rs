macro_rules! deps {
    () => {
        Reference!();
        TryFromBytes!();
        Maybe!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        unsafe impl < T : TryFromBytes + ? Sized > TryFromBytes for UnsafeCell < T > { # [allow (clippy :: missing_inline_in_public_items)] fn only_derive_is_allowed_to_implement_this_trait () where Self : Sized , { } # [inline] fn is_bit_valid < A : invariant :: Reference > (candidate : Maybe < '_ , Self , A >) -> bool { let c = candidate . into_exclusive_or_pme () ; T :: is_bit_valid (c . get_mut ()) } }
    };
}

impl_274!();