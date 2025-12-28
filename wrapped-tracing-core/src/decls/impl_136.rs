macro_rules! impl_136 {
    () => {
        # [cfg (feature = "std")] impl crate :: sealed :: Sealed for dyn std :: error :: Error + Sync + 'static { }
    };
}

impl_136!()