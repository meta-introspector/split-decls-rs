macro_rules! deps {
    () => {
        PrimitiveRepr!();
        Repr!();
    };
}

macro_rules! EnumRepr {
    () => {
        deps!();
        # [doc = " The representations which can legally appear on an enum type."] pub (crate) type EnumRepr = Repr < PrimitiveRepr , Infallible > ;
    };
}

EnumRepr!();