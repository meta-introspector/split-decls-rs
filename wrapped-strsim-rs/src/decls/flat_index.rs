macro_rules! flat_index {
    () => {
        fn flat_index (i : usize , j : usize , width : usize) -> usize { j * width + i }
    };
}

flat_index!();