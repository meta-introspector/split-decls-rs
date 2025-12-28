macro_rules! deps {
    () => {
        InvalidAttrAtCrateLevel!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < G : EmissionGuarantee > Diagnostic < '_ , G > for InvalidAttrAtCrateLevel { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let mut diag = Diag :: new (dcx , level , fluent :: passes_invalid_attr_at_crate_level) ; diag . span (self . span) ; diag . arg ("name" , self . name) ; if let Some (span) = self . sugg_span { diag . span_suggestion_verbose (span , fluent :: passes_suggestion , String :: new () , Applicability :: MachineApplicable ,) ; } if let Some (item) = self . item { diag . arg ("kind" , item . kind) ; diag . span_label (item . span , fluent :: passes_invalid_attr_at_crate_level_item) ; } diag } }
    };
}

impl_163!()