macro_rules! deps {
    () => {
        Repr!();
        PrimitiveRepr!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < Prim , Packed > Repr < Prim , Packed > { pub (crate) fn from_attrs (attrs : & [Attribute]) -> Result < Repr < Prim , Packed > , Error > where Prim : With < PrimitiveRepr > , Packed : With < NonZeroU32 > , { Repr :: from_attrs_inner (attrs) . map_err (Into :: into) } }
    };
}

impl_53!();