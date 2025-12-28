macro_rules! FnItemRef {
    () => {
        # [derive (LintDiagnostic)] # [diag (mir_transform_fn_item_ref)] pub (crate) struct FnItemRef { # [suggestion (code = "{sugg}" , applicability = "unspecified")] pub span : Span , pub sugg : String , pub ident : Ident , }
    };
}

FnItemRef!()