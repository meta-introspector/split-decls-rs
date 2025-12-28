macro_rules! PR_GET_NAME {
    () => {
        # [cfg (feature = "alloc")] const PR_GET_NAME : c_int = 16 ;
    };
}

PR_GET_NAME!()