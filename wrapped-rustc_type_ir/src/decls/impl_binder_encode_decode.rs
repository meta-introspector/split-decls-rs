macro_rules! deps {
    () => {
        Interner!();
        Binder!();
        TypeVisitable!();
    };
}

macro_rules! impl_binder_encode_decode {
    () => {
        deps!();
        # [cfg (feature = "nightly")] macro_rules ! impl_binder_encode_decode { ($ ($ t : ty) ,+ $ (,) ?) => { $ (impl < I : Interner , E : rustc_serialize :: Encoder > rustc_serialize :: Encodable < E > for ty :: Binder < I , $ t > where $ t : rustc_serialize :: Encodable < E >, I :: BoundVarKinds : rustc_serialize :: Encodable < E >, { fn encode (& self , e : & mut E) { self . bound_vars () . encode (e) ; self . as_ref () . skip_binder () . encode (e) ; } } impl < I : Interner , D : rustc_serialize :: Decoder > rustc_serialize :: Decodable < D > for ty :: Binder < I , $ t > where $ t : TypeVisitable < I > + rustc_serialize :: Decodable < D >, I :: BoundVarKinds : rustc_serialize :: Decodable < D >, { fn decode (decoder : & mut D) -> Self { let bound_vars = rustc_serialize :: Decodable :: decode (decoder) ; ty :: Binder :: bind_with_vars (rustc_serialize :: Decodable :: decode (decoder) , bound_vars) } }) * } }
    };
}

impl_binder_encode_decode!()