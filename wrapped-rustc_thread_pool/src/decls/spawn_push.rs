macro_rules! spawn_push {
    () => {
        macro_rules ! spawn_push { ($ scope : ident . $ spawn : ident , $ vec : ident , $ i : expr) => { { $ scope .$ spawn (move | _ | $ vec . lock () . unwrap () . push ($ i)) ; } } ; }
    };
}

spawn_push!();