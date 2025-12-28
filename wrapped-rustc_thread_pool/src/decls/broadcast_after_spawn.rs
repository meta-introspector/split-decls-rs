macro_rules! broadcast_after_spawn {
    () => {
        # [test] fn broadcast_after_spawn () { let (tx , rx) = channel () ; crate :: registry :: in_worker (move | _ , _ | { crate :: spawn (move | | tx . send (22) . unwrap ()) ; }) ; crate :: broadcast (| _ | { }) ; assert_eq ! (22 , rx . try_recv () . unwrap ()) ; }
    };
}

broadcast_after_spawn!();