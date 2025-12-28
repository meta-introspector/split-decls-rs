macro_rules! impl_138 {
    () => {
        # [cfg (feature = "std")] impl crate :: sealed :: Sealed for dyn std :: error :: Error + Send + Sync + 'static { }
    };
}

impl_138!()