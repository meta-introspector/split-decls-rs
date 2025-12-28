macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! AsDynError {
    () => {
        deps!();
        # [doc (hidden)] pub trait AsDynError < 'a > : Sealed { fn as_dyn_error (& self) -> & (dyn Error + 'a) ; }
    };
}

AsDynError!();