macro_rules! deps {
    () => {
        ReadBuf!();
    };
}

macro_rules! deref_async_read {
    () => {
        deps!();
        macro_rules ! deref_async_read { () => { fn poll_read (mut self : Pin <& mut Self >, cx : & mut Context <'_ >, buf : & mut ReadBuf <'_ >,) -> Poll < io :: Result < () >> { Pin :: new (& mut ** self) . poll_read (cx , buf) } } ; }
    };
}

deref_async_read!()