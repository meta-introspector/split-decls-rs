macro_rules! deps {
    () => {
        DataExt!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl DataExt for Data { fn fields (& self) -> Vec < (& Visibility , TokenStream , & Type) > { match self { Data :: Struct (strc) => strc . fields () , Data :: Enum (enm) => enm . fields () , Data :: Union (un) => un . fields () , } } fn variants (& self) -> Vec < Vec < (& Visibility , TokenStream , & Type) > > { match self { Data :: Struct (strc) => strc . variants () , Data :: Enum (enm) => enm . variants () , Data :: Union (un) => un . variants () , } } fn tag (& self) -> Option < Ident > { match self { Data :: Struct (strc) => strc . tag () , Data :: Enum (enm) => enm . tag () , Data :: Union (un) => un . tag () , } } }
    };
}

impl_2!();