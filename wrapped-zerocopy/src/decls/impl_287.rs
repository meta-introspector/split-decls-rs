macro_rules! deps {
    () => {
        SizeInfo!();
        SliceDst!();
        TrailingSliceLayout!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl SizeInfo { # [doc = " Attempts to create a `SizeInfo` from `Self` in which `elem_size` is a"] # [doc = " `NonZeroUsize`. If `elem_size` is 0, returns `None`."] # [allow (unused)] const fn try_to_nonzero_elem_size (& self) -> Option < SizeInfo < NonZeroUsize > > { Some (match * self { SizeInfo :: Sized { size } => SizeInfo :: Sized { size } , SizeInfo :: SliceDst (TrailingSliceLayout { offset , elem_size }) => { if let Some (elem_size) = NonZeroUsize :: new (elem_size) { SizeInfo :: SliceDst (TrailingSliceLayout { offset , elem_size }) } else { return None ; } } }) } }
    };
}

impl_287!()