macro_rules! deps {
    () => {
        SourceFile!();
        StableSourceFileId!();
    };
}

macro_rules! SourceMapFiles {
    () => {
        deps!();
        # [derive (Default)] struct SourceMapFiles { source_files : monotonic :: MonotonicVec < Arc < SourceFile > > , stable_id_to_source_file : UnhashMap < StableSourceFileId , Arc < SourceFile > > , }
    };
}

SourceMapFiles!();