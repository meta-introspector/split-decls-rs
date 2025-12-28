macro_rules! deps {
    () => {
        TestOutput!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < G : EmissionGuarantee > Diagnostic < '_ , G > for TestOutput { fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let TestOutput { span , kind , content } = self ; # [allow (rustc :: untranslatable_diagnostic)] Diag :: new (dcx , level , format ! ("{kind}({content})")) . with_span (span) } }
    };
}

impl_37!()