macro_rules! deps {
    () => {
        Project!();
    };
}

macro_rules! cargo_target_dir {
    () => {
        deps!();
        fn cargo_target_dir (project : & Project) -> impl Iterator < Item = (& 'static str , PathBuf) > { iter :: once (("CARGO_TARGET_DIR" , path ! (project . target_dir / "tests" / "trybuild") ,)) }
    };
}

cargo_target_dir!();