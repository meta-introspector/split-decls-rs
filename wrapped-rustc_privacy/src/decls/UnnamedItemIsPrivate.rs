macro_rules! UnnamedItemIsPrivate {
    () => {
        # [derive (Diagnostic)] # [diag (privacy_unnamed_item_is_private)] pub (crate) struct UnnamedItemIsPrivate { # [primary_span] pub span : Span , pub kind : & 'static str , }
    };
}

UnnamedItemIsPrivate!()