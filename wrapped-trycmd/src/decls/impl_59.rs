macro_rules! deps {
    () => {
        Spawn!();
        SpawnStatus!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl Default for Spawn { fn default () -> Self { Self { exit : None , status : SpawnStatus :: Skipped , } } }
    };
}

impl_59!()