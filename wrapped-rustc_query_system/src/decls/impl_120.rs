macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < 'a > rustc_span :: HashStableContext for StableHashingContext < 'a > { # [inline] fn hash_spans (& self) -> bool { self . hashing_controls . hash_spans } # [inline] fn unstable_opts_incremental_ignore_spans (& self) -> bool { self . incremental_ignore_spans } # [inline] fn def_path_hash (& self , def_id : DefId) -> DefPathHash { self . def_path_hash (def_id) } # [inline] fn def_span (& self , def_id : LocalDefId) -> Span { self . untracked . source_span . get (def_id) . unwrap_or (DUMMY_SP) } # [inline] fn span_data_to_lines_and_cols (& mut self , span : & SpanData ,) -> Option < (StableSourceFileId , usize , BytePos , usize , BytePos) > { self . source_map () . span_data_to_lines_and_cols (span) } # [inline] fn hashing_controls (& self) -> HashingControls { self . hashing_controls . clone () } }
    };
}

impl_120!();