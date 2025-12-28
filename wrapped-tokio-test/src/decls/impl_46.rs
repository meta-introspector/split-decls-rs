macro_rules! deps {
    () => {
        Spawn!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T : Stream > Spawn < T > { # [doc = " If `T` is a [`Stream`] then `poll_next` it. This will handle pinning and the context"] # [doc = " type for the stream."] pub fn poll_next (& mut self) -> Poll < Option < T :: Item > > { let stream = self . future . as_mut () ; self . task . enter (| cx | stream . poll_next (cx)) } }
    };
}

impl_46!()