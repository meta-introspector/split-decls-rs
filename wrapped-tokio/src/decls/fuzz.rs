macro_rules! fuzz {
    () => {
        # [doc = " fuzz test (`fuzz_linked_list`)"] # [cfg (fuzzing)] pub mod fuzz ;
    };
}

fuzz!();