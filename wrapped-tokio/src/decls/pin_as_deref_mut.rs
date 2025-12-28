macro_rules! pin_as_deref_mut {
    () => {
        # [doc = " Copy of [`std::pin::Pin::as_deref_mut`]."] pub (crate) fn pin_as_deref_mut < P : DerefMut > (ptr : Pin < & mut Pin < P > >) -> Pin < & mut P :: Target > { unsafe { ptr . get_unchecked_mut () } . as_mut () }
    };
}

pin_as_deref_mut!()