macro_rules! deps {
    () => {
        UnusedVariableStringInterp!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl Subdiagnostic for UnusedVariableStringInterp { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . span_label (self . lit , crate :: fluent_generated :: passes_maybe_string_interpolation) ; diag . multipart_suggestion (crate :: fluent_generated :: passes_string_interpolation_only_works , vec ! [(self . lo , String :: from ("format!(")) , (self . hi , String :: from (")"))] , Applicability :: MachineApplicable ,) ; } }
    };
}

impl_229!();