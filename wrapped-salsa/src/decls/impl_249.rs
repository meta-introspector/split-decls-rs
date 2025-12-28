macro_rules! deps {
    () => {
        Revision!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl Revision { # [inline] pub (crate) fn max () -> Self { Self :: from (usize :: MAX) } # [inline] pub (crate) const fn start () -> Self { Self { generation : unsafe { NonZeroUsize :: new_unchecked (START) } , } } # [inline] pub (crate) fn from (g : usize) -> Self { Self { generation : NonZeroUsize :: new (g) . unwrap () , } } # [inline] pub (crate) fn from_opt (g : usize) -> Option < Self > { NonZeroUsize :: new (g) . map (| generation | Self { generation }) } # [inline] pub (crate) fn next (self) -> Revision { Self :: from (self . generation . get () + 1) } # [inline] pub (crate) fn as_usize (self) -> usize { self . generation . get () } }
    };
}

impl_249!()