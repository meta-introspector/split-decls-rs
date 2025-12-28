macro_rules! Message {
    () => {
        # [derive (Debug)] enum Message { Config (loader :: Config) , Invalidate (AbsPathBuf) , }
    };
}

Message!()