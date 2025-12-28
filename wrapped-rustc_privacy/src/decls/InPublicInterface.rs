macro_rules! InPublicInterface {
    () => {
        # [derive (Diagnostic)] # [diag (privacy_in_public_interface , code = E0446)] pub (crate) struct InPublicInterface < 'a > { # [primary_span] # [label] pub span : Span , pub vis_descr : & 'static str , pub kind : & 'a str , pub descr : DiagArgFromDisplay < 'a > , # [label (privacy_visibility_label)] pub vis_span : Span , }
    };
}

InPublicInterface!()