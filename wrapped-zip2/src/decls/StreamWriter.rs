macro_rules! StreamWriter {
    () => {
        # [doc = " Wrapper around a [Write] implementation that implements the [Seek] trait, but where seeking"] # [doc = " returns an error unless it's a no-op."] pub struct StreamWriter < W : Write > { inner : W , bytes_written : u64 , }
    };
}

StreamWriter!();