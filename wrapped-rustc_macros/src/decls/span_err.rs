macro_rules! span_err {
    () => {
        # [doc = " Returns an error diagnostic on span `span` with msg `msg`."] # [must_use] pub (crate) fn span_err < T : Into < String > > (span : impl MultiSpan , msg : T) -> Diagnostic { Diagnostic :: spanned (span , Level :: Error , format ! ("derive(Diagnostic): {}" , msg . into ())) }
    };
}

span_err!();