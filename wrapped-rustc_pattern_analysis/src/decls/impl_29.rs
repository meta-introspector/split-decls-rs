macro_rules! deps {
    () => {
        Overlap!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Subdiagnostic for Overlap { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let Overlap { span , range } = self ; let message = format ! ("this range overlaps on `{range}`...") ; diag . span_label (span , message) ; } }
    };
}

impl_29!();