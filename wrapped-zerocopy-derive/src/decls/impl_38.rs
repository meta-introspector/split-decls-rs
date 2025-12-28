macro_rules! deps {
    () => {
        CompoundRepr!();
        PrimitiveRepr!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < Prim : With < PrimitiveRepr > + Copy > ToTokens for Spanned < CompoundRepr < Prim > > { fn to_tokens (& self , ts : & mut TokenStream) { use CompoundRepr :: * ; match & self . t { C => ts . append_all (quote_spanned ! { self . span => # [repr (C)] }) , Rust => ts . append_all (quote_spanned ! { self . span => # [repr (Rust)] }) , Primitive (prim) => prim . with (| prim | Spanned :: new (prim , self . span) . to_tokens (ts)) , } } }
    };
}

impl_38!();