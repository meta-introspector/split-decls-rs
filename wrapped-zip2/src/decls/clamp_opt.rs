macro_rules! clamp_opt {
    () => {
        # [cfg (any (feature = "_deflate-any" , feature = "bzip2" , feature = "ppmd" , feature = "xz" , feature = "zstd" ,))] fn clamp_opt < T : Ord + Copy , U : Ord + Copy + TryFrom < T > > (value : T , range : std :: ops :: RangeInclusive < U > ,) -> Option < T > { if range . contains (& value . try_into () . ok () ?) { Some (value) } else { None } }
    };
}

clamp_opt!();