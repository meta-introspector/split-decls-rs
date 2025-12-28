macro_rules! deps {
    () => {
        RawDir!();
    };
}

macro_rules! Reader {
    () => {
        deps!();
        # [doc = " An inotify event iterator implemented with the read syscall."] # [doc = ""] # [doc = " See the [`RawDir`] API for more details and usage examples as this API is"] # [doc = " based on it."] # [doc = ""] # [doc = " [`RawDir`]: crate::fs::raw_dir::RawDir"] pub struct Reader < 'buf , Fd : AsFd > { fd : Fd , buf : & 'buf mut [MaybeUninit < u8 >] , initialized : usize , offset : usize , }
    };
}

Reader!();