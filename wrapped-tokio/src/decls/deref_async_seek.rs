macro_rules! deref_async_seek {
    () => {
        macro_rules ! deref_async_seek { () => { fn start_seek (mut self : Pin <& mut Self >, pos : SeekFrom) -> io :: Result < () > { Pin :: new (& mut ** self) . start_seek (pos) } fn poll_complete (mut self : Pin <& mut Self >, cx : & mut Context <'_ >) -> Poll < io :: Result < u64 >> { Pin :: new (& mut ** self) . poll_complete (cx) } } ; }
    };
}

deref_async_seek!()