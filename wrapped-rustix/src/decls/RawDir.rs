macro_rules! RawDir {
    () => {
        # [doc = " A directory iterator implemented with getdents."] # [doc = ""] # [doc = " Note: This implementation does not handle growing the buffer. If this"] # [doc = " functionality is necessary, you'll need to drop the current iterator,"] # [doc = " resize the buffer, and then re-create the iterator. The iterator is"] # [doc = " guaranteed to continue where it left off provided the file descriptor isn't"] # [doc = " changed. See the example in [`RawDir::new`]."] pub struct RawDir < 'buf , Fd : AsFd > { fd : Fd , buf : & 'buf mut [MaybeUninit < u8 >] , initialized : usize , offset : usize , }
    };
}

RawDir!()