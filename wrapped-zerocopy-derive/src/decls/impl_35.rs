macro_rules! deps {
    () => {
        Repr!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < Prim > Repr < Prim , NonZeroU32 > { # [doc = " Does `self` describe a `#[repr(packed)]` or `#[repr(packed(1))]` type?"] pub (crate) fn is_packed_1 (& self) -> bool { self . get_packed () . map (| n | n . get () == 1) . unwrap_or (false) } }
    };
}

impl_35!()