macro_rules! deps {
    () => {
        Repr!();
    };
}

macro_rules! StructUnionRepr {
    () => {
        deps!();
        # [doc = " The representations which can legally appear on a struct or union type."] pub (crate) type StructUnionRepr = Repr < Infallible , NonZeroU32 > ;
    };
}

StructUnionRepr!()