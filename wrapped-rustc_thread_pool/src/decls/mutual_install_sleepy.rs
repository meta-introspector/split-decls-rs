macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! mutual_install_sleepy {
    () => {
        deps!();
        # [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn mutual_install_sleepy () { use std :: { thread , time } ; let pool1 = ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap () ; let pool2 = ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap () ; let ok = pool1 . install (| | { pool2 . install (| | { thread :: sleep (time :: Duration :: from_secs (1)) ; pool1 . install (| | { thread :: sleep (time :: Duration :: from_secs (1)) ; true }) }) }) ; assert ! (ok) ; }
    };
}

mutual_install_sleepy!();