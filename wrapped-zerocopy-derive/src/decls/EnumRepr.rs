macro_rules! deps {
    () => {
        Repr!();
        PrimitiveRepr!();
    };
}

macro_rules! EnumRepr {
    () => {
        deps!();
        # [doc = " The representations which can legally appear on an enum type."] pub (crate) type EnumRepr = Repr < PrimitiveRepr , Infallible > ;
    };
}

EnumRepr!()