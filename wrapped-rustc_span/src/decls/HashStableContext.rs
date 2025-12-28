macro_rules! deps {
    () => {
        StableSourceFileId!();
        LocalDefId!();
        Span!();
        SpanData!();
        DefId!();
        DefPathHash!();
    };
}

macro_rules! HashStableContext {
    () => {
        deps!();
        # [doc = " Requirements for a `StableHashingContext` to be used in this crate."] # [doc = ""] # [doc = " This is a hack to allow using the [`HashStable_Generic`] derive macro"] # [doc = " instead of implementing everything in rustc_middle."] pub trait HashStableContext { fn def_path_hash (& self , def_id : DefId) -> DefPathHash ; fn hash_spans (& self) -> bool ; # [doc = " Accesses `sess.opts.unstable_opts.incremental_ignore_spans` since"] # [doc = " we don't have easy access to a `Session`"] fn unstable_opts_incremental_ignore_spans (& self) -> bool ; fn def_span (& self , def_id : LocalDefId) -> Span ; fn span_data_to_lines_and_cols (& mut self , span : & SpanData ,) -> Option < (StableSourceFileId , usize , BytePos , usize , BytePos) > ; fn hashing_controls (& self) -> HashingControls ; }
    };
}

HashStableContext!()