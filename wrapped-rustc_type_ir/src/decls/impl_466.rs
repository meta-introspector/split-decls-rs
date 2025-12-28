macro_rules! deps {
    () => {
        Interner!();
        Ty!();
        UnsafeBinderInner!();
    };
}

macro_rules! impl_466 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl < I : Interner , E : rustc_serialize :: Encoder > rustc_serialize :: Encodable < E > for UnsafeBinderInner < I > where I :: Ty : rustc_serialize :: Encodable < E > , I :: BoundVarKinds : rustc_serialize :: Encodable < E > , { fn encode (& self , e : & mut E) { self . bound_vars () . encode (e) ; self . as_ref () . skip_binder () . encode (e) ; } }
    };
}

impl_466!();