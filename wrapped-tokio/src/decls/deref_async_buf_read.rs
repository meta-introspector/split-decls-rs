macro_rules! deref_async_buf_read {
    () => {
        macro_rules ! deref_async_buf_read { () => { fn poll_fill_buf (self : Pin <& mut Self >, cx : & mut Context <'_ >) -> Poll < io :: Result <& [u8] >> { Pin :: new (& mut ** self . get_mut ()) . poll_fill_buf (cx) } fn consume (mut self : Pin <& mut Self >, amt : usize) { Pin :: new (& mut ** self) . consume (amt) } } ; }
    };
}

deref_async_buf_read!();