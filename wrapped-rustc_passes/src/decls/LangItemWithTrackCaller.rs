macro_rules! LangItemWithTrackCaller {
    () => {
        # [derive (Diagnostic)] # [diag (passes_lang_item_fn_with_track_caller)] pub (crate) struct LangItemWithTrackCaller { # [primary_span] pub attr_span : Span , pub name : Symbol , # [label] pub sig_span : Span , }
    };
}

LangItemWithTrackCaller!();