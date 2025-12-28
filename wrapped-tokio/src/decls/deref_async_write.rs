macro_rules! deref_async_write {
    () => {
        macro_rules ! deref_async_write { () => { fn poll_write (mut self : Pin <& mut Self >, cx : & mut Context <'_ >, buf : & [u8] ,) -> Poll < io :: Result < usize >> { Pin :: new (& mut ** self) . poll_write (cx , buf) } fn poll_write_vectored (mut self : Pin <& mut Self >, cx : & mut Context <'_ >, bufs : & [IoSlice <'_ >] ,) -> Poll < io :: Result < usize >> { Pin :: new (& mut ** self) . poll_write_vectored (cx , bufs) } fn is_write_vectored (& self) -> bool { (** self) . is_write_vectored () } fn poll_flush (mut self : Pin <& mut Self >, cx : & mut Context <'_ >) -> Poll < io :: Result < () >> { Pin :: new (& mut ** self) . poll_flush (cx) } fn poll_shutdown (mut self : Pin <& mut Self >, cx : & mut Context <'_ >) -> Poll < io :: Result < () >> { Pin :: new (& mut ** self) . poll_shutdown (cx) } } ; }
    };
}

deref_async_write!();