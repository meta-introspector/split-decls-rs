macro_rules! cfg_not_test_util {
    () => {
        macro_rules ! cfg_not_test_util { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "test-util"))] $ item) * } }
    };
}

cfg_not_test_util!()