macro_rules! raw_cargo {
    () => {
        fn raw_cargo () -> Command { match env :: var_os ("CARGO") { Some (cargo) => Command :: new (cargo) , None => Command :: new ("cargo") , } }
    };
}

raw_cargo!()