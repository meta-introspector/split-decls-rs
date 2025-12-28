macro_rules! ZipWriterStats {
    () => {
        # [derive (Default , Debug)] struct ZipWriterStats { hasher : Hasher , start : u64 , bytes_written : u64 , }
    };
}

ZipWriterStats!()