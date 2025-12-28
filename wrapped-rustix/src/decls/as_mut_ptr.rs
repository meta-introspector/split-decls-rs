macro_rules! as_mut_ptr {
    () => {
        # [doc = " Convert a `&mut T` into a `*mut T` without using an `as`."] # [inline] pub (crate) fn as_mut_ptr < T > (t : & mut T) -> * mut T { t }
    };
}

as_mut_ptr!();