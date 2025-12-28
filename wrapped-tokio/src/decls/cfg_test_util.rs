macro_rules! cfg_test_util {
    () => {
        macro_rules ! cfg_test_util { ($ ($ item : item) *) => { $ (# [cfg (feature = "test-util")] # [cfg_attr (docsrs , doc (cfg (feature = "test-util")))] $ item) * } }
    };
}

cfg_test_util!()