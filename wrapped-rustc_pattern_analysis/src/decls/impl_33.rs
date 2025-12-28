macro_rules! deps {
    () => {
        GappedRange!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Subdiagnostic for GappedRange { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let GappedRange { span , gap , first_range } = self ; let message = format ! ("this could appear to continue range `{first_range}`, but `{gap}` isn't matched by \
            either of them") ; diag . span_label (span , message) ; } }
    };
}

impl_33!();