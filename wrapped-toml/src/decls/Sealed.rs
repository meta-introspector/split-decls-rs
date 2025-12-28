macro_rules! Sealed {
    () => {
        # [doc = " An implementation detail that should not be implemented, this will change in"] # [doc = " the future and break code otherwise."] # [doc (hidden)] pub trait Sealed { }
    };
}

Sealed!();