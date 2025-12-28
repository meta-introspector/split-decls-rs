macro_rules! deps {
    () => {
        DeflateStream!();
    };
}

macro_rules! flush_block_only {
    () => {
        deps!();
        pub (crate) fn flush_block_only (stream : & mut DeflateStream , is_last : bool) { zng_tr_flush_block (stream , (stream . state . block_start >= 0) . then_some (stream . state . block_start as usize) , (stream . state . strstart as isize - stream . state . block_start) as u32 , is_last ,) ; stream . state . block_start = stream . state . strstart as isize ; flush_pending (stream) }
    };
}

flush_block_only!();