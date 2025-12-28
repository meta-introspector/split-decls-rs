macro_rules! deps {
    () => {
        MatchGeneratorDriver!();
        MatchGenerator!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl MatchGeneratorDriver { # [doc = " slice_size says how big the slices should be that are allocated to work with"] # [doc = " max_slices_in_window says how many slices should at most be used while looking for matches"] pub (crate) fn new (slice_size : usize , max_slices_in_window : usize) -> Self { Self { vec_pool : Vec :: new () , suffix_pool : Vec :: new () , match_generator : MatchGenerator :: new (max_slices_in_window * slice_size) , slice_size , } } }
    };
}

impl_249!()