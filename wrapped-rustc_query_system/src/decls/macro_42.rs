macro_rules! macro_42 {
    () => {
        rustc_data_structures :: static_assert_size ! (Option < DepNodeIndex >, 4) ;
    };
}

macro_42!();