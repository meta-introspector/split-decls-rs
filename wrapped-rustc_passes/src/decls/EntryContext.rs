macro_rules! EntryContext {
    () => {
        struct EntryContext < 'tcx > { tcx : TyCtxt < 'tcx > , # [doc = " The function has the `#[rustc_main]` attribute."] rustc_main_fn : Option < (LocalDefId , Span) > , # [doc = " The functions that one might think are `main` but aren't, e.g."] # [doc = " main functions not defined at the top level. For diagnostics."] non_main_fns : Vec < Span > , }
    };
}

EntryContext!();