macro_rules! deps {
    () => {
        FieldInnerTy!();
    };
}

macro_rules! FieldInfo {
    () => {
        deps!();
        # [doc = " Field information passed to the builder. Deliberately omits attrs to discourage the"] # [doc = " `generate_*` methods from walking the attributes themselves."] pub (crate) struct FieldInfo < 'a > { pub (crate) binding : & 'a BindingInfo < 'a > , pub (crate) ty : FieldInnerTy < 'a > , pub (crate) span : & 'a proc_macro2 :: Span , }
    };
}

FieldInfo!()