macro_rules! ReadmeDoctests {
    () => {
        # [doc = include_str ! ("../README.md")] # [cfg (doctest)] pub struct ReadmeDoctests ;
    };
}

ReadmeDoctests!();