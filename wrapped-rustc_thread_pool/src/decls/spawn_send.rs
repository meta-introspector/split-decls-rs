macro_rules! spawn_send {
    () => {
        macro_rules ! spawn_send { ($ spawn : ident , $ tx : ident , $ i : expr) => { { let tx = $ tx . clone () ; $ spawn (move || tx . send ($ i) . unwrap ()) ; } } ; }
    };
}

spawn_send!()