macro_rules! secrecy {
    () => {
        # [cfg (feature = "secrecy")] mod secrecy ;
    };
}

secrecy!();