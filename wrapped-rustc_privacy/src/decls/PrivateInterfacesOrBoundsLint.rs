macro_rules! PrivateInterfacesOrBoundsLint {
    () => {
        # [derive (LintDiagnostic)] # [diag (privacy_private_interface_or_bounds_lint)] pub (crate) struct PrivateInterfacesOrBoundsLint < 'a > { # [label (privacy_item_label)] pub item_span : Span , pub item_kind : & 'a str , pub item_descr : DiagArgFromDisplay < 'a > , pub item_vis_descr : & 'a str , # [note (privacy_ty_note)] pub ty_span : Span , pub ty_kind : & 'a str , pub ty_descr : DiagArgFromDisplay < 'a > , pub ty_vis_descr : & 'a str , }
    };
}

PrivateInterfacesOrBoundsLint!()