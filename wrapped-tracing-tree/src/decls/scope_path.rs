macro_rules! scope_path {
    () => {
        fn scope_path < 'a , R : LookupSpan < 'a > > (span : & SpanRef < 'a , R >) -> ScopeFromRoot < 'a , R > { span . scope () . from_root () }
    };
}

scope_path!()