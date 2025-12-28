macro_rules! deps {
    () => {
        ArgAttributes!();
        ArgExtension!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl From < ArgAttribute > for ArgAttributes { fn from (value : ArgAttribute) -> Self { Self { regular : value , arg_ext : ArgExtension :: None , pointee_size : Size :: ZERO , pointee_align : None , } } }
    };
}

impl_273!();