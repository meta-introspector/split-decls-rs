macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl StdError for Error { fn source (& self) -> Option < & (dyn StdError + 'static) > { self . source . as_ref () . map (| c | & * * c as & (dyn StdError + 'static)) } }
    };
}

impl_117!()