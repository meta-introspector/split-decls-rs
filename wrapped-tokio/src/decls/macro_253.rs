macro_rules! macro_253 {
    () => {
        cfg_rt ! { pub (crate) use crate :: runtime :: spawn_blocking ; cfg_fs ! { # [allow (unused_imports)] pub (crate) use crate :: runtime :: spawn_mandatory_blocking ; } pub (crate) use crate :: task :: JoinHandle ; }
    };
}

macro_253!()