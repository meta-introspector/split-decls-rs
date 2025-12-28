macro_rules! deps {
    () => {
        SpawnStatus!();
        Spawn!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl Default for Spawn { fn default () -> Self { Self { exit : None , status : SpawnStatus :: Skipped , } } }
    };
}

impl_59!();