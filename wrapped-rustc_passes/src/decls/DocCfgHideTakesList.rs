macro_rules! DocCfgHideTakesList {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_cfg_hide_takes_list)] pub (crate) struct DocCfgHideTakesList ;
    };
}

DocCfgHideTakesList!()