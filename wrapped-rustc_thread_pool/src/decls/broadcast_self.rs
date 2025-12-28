macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! broadcast_self {
    () => {
        deps!();
        # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn broadcast_self () { let pool = ThreadPoolBuilder :: new () . num_threads (7) . build () . unwrap () ; let v = pool . install (| | crate :: broadcast (| ctx | ctx . index ())) ; assert ! (v . into_iter () . eq (0 .. 7)) ; }
    };
}

broadcast_self!()