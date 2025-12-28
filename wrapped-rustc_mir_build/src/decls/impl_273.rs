macro_rules! deps {
    () => {
        AdtDefinedHere!();
        Variant!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < 'tcx > Subdiagnostic for AdtDefinedHere < 'tcx > { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . arg ("ty" , self . ty) ; let mut spans = MultiSpan :: from (self . adt_def_span) ; for Variant { span } in self . variants { spans . push_span_label (span , fluent :: mir_build_variant_defined_here) ; } diag . span_note (spans , fluent :: mir_build_adt_defined_here) ; } }
    };
}

impl_273!()