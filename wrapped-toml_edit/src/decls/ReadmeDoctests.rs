macro_rules! ReadmeDoctests {
    () => {
        # [doc = include_str ! ("../README.md")] # [cfg (doctest)] # [cfg (feature = "display")] # [cfg (feature = "parse")] pub struct ReadmeDoctests ;
    };
}

ReadmeDoctests!();