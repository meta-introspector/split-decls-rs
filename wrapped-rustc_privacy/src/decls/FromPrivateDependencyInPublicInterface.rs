macro_rules! FromPrivateDependencyInPublicInterface {
    () => {
        # [derive (LintDiagnostic)] # [diag (privacy_from_private_dep_in_public_interface)] pub (crate) struct FromPrivateDependencyInPublicInterface < 'a > { pub kind : & 'a str , pub descr : DiagArgFromDisplay < 'a > , pub krate : Symbol , }
    };
}

FromPrivateDependencyInPublicInterface!();