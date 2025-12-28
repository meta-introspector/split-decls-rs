macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl core :: error :: Error for Error { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { # [cfg (not (feature = "alloc"))] { None } # [cfg (feature = "alloc")] { self . source . as_ref () . map (| source | source . as_ref () as & (dyn core :: error :: Error + 'static)) } } }
    };
}

impl_15!();