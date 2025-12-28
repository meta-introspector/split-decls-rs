macro_rules! url {
    () => {
        # [cfg (feature = "url")] mod url ;
    };
}

url!()