macro_rules! as_ptr {
    () => {
        # [doc = " Convert a `&T` into a `*const T` without using an `as`."] # [inline] pub (crate) const fn as_ptr < T > (t : & T) -> * const T { t }
    };
}

as_ptr!();