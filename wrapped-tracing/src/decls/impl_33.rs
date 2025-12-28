macro_rules! deps {
    () => {
        AsField!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl AsField for & Field { # [inline] fn as_field (& self , metadata : & Metadata < '_ >) -> Option < Field > { if self . callsite () == metadata . callsite () { Some ((* self) . clone ()) } else { None } } }
    };
}

impl_33!()