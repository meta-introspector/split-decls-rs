macro_rules! BindingsWithVariantName {
    () => {
        # [derive (LintDiagnostic)] # [diag (mir_build_bindings_with_variant_name , code = E0170)] pub (crate) struct BindingsWithVariantName { # [suggestion (code = "{ty_path}::{name}" , applicability = "machine-applicable")] pub (crate) suggestion : Option < Span > , pub (crate) ty_path : String , pub (crate) name : Ident , }
    };
}

BindingsWithVariantName!();