macro_rules! deps {
    () => {
        Handle!();
        HandleKind!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl IntoRawHandle for crate :: Handle { fn into_raw_handle (self) -> RawHandle { match self . 0 . kind { HandleKind :: Owned (h) => h . into_raw_handle () , HandleKind :: Borrowed (h) => h . as_raw_handle () , } } }
    };
}

impl_23!()