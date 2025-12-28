macro_rules! deps {
    () => {
        ActualAncestry!();
        HasAncestry!();
    };
}

macro_rules! context_get_ancestry {
    () => {
        deps!();
        fn context_get_ancestry < C > (item : impl HasAncestry , ctx : & Context < '_ , C >) -> ActualAncestry where C : Subscriber + for < 'a > LookupSpan < 'a > , { get_ancestry (item , | | ctx . lookup_current () . map (| s | s . id ()) , | span_id | ctx . span (span_id) . map (| span | (& span) . into ()) ,) }
    };
}

context_get_ancestry!();