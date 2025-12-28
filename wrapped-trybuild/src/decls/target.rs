macro_rules! target {
    () => {
        fn target () -> Vec < & 'static str > { if cfg ! (trybuild_no_target) { vec ! [] } else { vec ! ["--target" , TARGET] } }
    };
}

target!()