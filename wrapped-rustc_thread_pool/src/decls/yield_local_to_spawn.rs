macro_rules! yield_local_to_spawn {
    () => {
        # [test] fn yield_local_to_spawn () { let (tx , rx) = channel () ; crate :: spawn (move | | tx . send (22) . unwrap ()) ; crate :: registry :: in_worker (move | _ , _ | { crate :: yield_local () ; }) ; assert_eq ! (22 , rx . recv () . unwrap ()) ; }
    };
}

yield_local_to_spawn!();