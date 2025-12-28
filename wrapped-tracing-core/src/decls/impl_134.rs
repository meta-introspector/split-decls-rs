macro_rules! impl_134 {
    () => {
        # [cfg (feature = "std")] impl crate :: sealed :: Sealed for dyn std :: error :: Error + Send + 'static { }
    };
}

impl_134!()