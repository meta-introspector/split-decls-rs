macro_rules! macro_437 {
    () => {
        # [rustfmt :: skip] impl_known_layout ! (T => Option < T >, T : ? Sized => PhantomData < T >, T => Wrapping < T >, T => CoreMaybeUninit < T >, T : ? Sized => * const T , T : ? Sized => * mut T , T : ? Sized => &'_ T , T : ? Sized => &'_ mut T ,) ;
    };
}

macro_437!()