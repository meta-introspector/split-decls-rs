macro_rules! deps {
    () => {
        Protocol!();
        RawProtocol!();
    };
}

macro_rules! impl_656 {
    () => {
        deps!();
        # [rustfmt :: skip] impl Protocol { # [doc = " Constructs a `Protocol` from a raw integer."] # [inline] pub const fn from_raw (raw : RawProtocol) -> Self { Self (raw) } # [doc = " Returns the raw integer for this `Protocol`."] # [inline] pub const fn as_raw (self) -> RawProtocol { self . 0 } }
    };
}

impl_656!()