macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! copy_stats {
    () => {
        deps!();
        # [cfg (not (feature = "dir"))] fn copy_stats (_source_meta : & std :: fs :: Metadata , _dest : & std :: path :: Path ,) -> Result < () , std :: io :: Error > { Ok (()) }
    };
}

copy_stats!()