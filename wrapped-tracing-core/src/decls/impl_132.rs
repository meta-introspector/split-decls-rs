macro_rules! impl_132 {
    () => {
        # [cfg (feature = "std")] impl crate :: sealed :: Sealed for dyn std :: error :: Error + 'static { }
    };
}

impl_132!();