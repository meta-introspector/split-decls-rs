macro_rules! deps {
    () => {
        PrimitiveRepr!();
        Repr!();
        CompoundRepr!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < Packed > Repr < PrimitiveRepr , Packed > { fn get_primitive (& self) -> Option < & PrimitiveRepr > { use CompoundRepr :: * ; use Repr :: * ; if let Compound (Spanned { t : Primitive (p) , span : _ } , _align) = self { Some (p) } else { None } } # [doc = " Does `self` describe a `#[repr(u8)]` type?"] pub (crate) fn is_u8 (& self) -> bool { matches ! (self . get_primitive () , Some (PrimitiveRepr :: U8)) } # [doc = " Does `self` describe a `#[repr(i8)]` type?"] pub (crate) fn is_i8 (& self) -> bool { matches ! (self . get_primitive () , Some (PrimitiveRepr :: I8)) } }
    };
}

impl_36!()