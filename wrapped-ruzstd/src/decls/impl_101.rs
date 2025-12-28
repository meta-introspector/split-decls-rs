macro_rules! deps {
    () => {
        FSETableError!();
        Error!();
        GetBitsError!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for FSETableError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { FSETableError :: GetBitsError (source) => Some (source) , _ => None , } } }
    };
}

impl_101!();