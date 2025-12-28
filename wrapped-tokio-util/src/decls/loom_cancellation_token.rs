macro_rules! loom_cancellation_token {
    () => {
        # [cfg (loom)] mod loom_cancellation_token ;
    };
}

loom_cancellation_token!()