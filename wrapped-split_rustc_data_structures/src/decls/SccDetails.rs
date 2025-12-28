macro_rules! SccDetails {
    () => {
        # [doc = " Information about an invidividual SCC node."] struct SccDetails { # [doc = " For this SCC, the range of `all_successors` where its"] # [doc = " successors can be found."] range : Range < usize > , }
    };
}

SccDetails!();