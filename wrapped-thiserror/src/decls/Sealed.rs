macro_rules! Sealed {
    () => {
        # [doc (hidden)] pub trait Sealed { }
    };
}

Sealed!()