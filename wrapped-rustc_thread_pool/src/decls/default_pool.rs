macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! default_pool {
    () => {
        deps!();
        # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn default_pool () { ThreadPoolBuilder :: default () . build () . unwrap () ; }
    };
}

default_pool!()