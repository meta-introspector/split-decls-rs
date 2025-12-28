macro_rules! deps {
    () => {
        VariantDef!();
        Symbol!();
        FieldDef!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        impl VariantDef { pub fn name (& self) -> Symbol { with (| cx | cx . variant_name (* self)) } # [doc = " Retrieve all the fields in this variant."] pub fn fields (& self) -> Vec < FieldDef > { with (| cx | cx . variant_fields (* self)) } }
    };
}

impl_361!();