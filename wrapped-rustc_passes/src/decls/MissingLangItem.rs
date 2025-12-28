macro_rules! MissingLangItem {
    () => {
        # [derive (Diagnostic)] # [diag (passes_missing_lang_item)] # [note] # [help] pub (crate) struct MissingLangItem { pub name : Symbol , }
    };
}

MissingLangItem!()