macro_rules! STACK_PER_RECURSION {
    () => {
        # [cfg (target_os = "aix")] const STACK_PER_RECURSION : usize = 16 * 1024 * 1024 ;
    };
}

STACK_PER_RECURSION!()