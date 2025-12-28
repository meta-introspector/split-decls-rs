macro_rules! deps {
    () => {
        HandleKind!();
        Handle!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl AsRawHandle for crate :: Handle { fn as_raw_handle (& self) -> RawHandle { match self . 0 . kind { HandleKind :: Owned (ref h) => h . as_raw_handle () , HandleKind :: Borrowed (ref h) => h . as_raw_handle () , } } }
    };
}

impl_22!();