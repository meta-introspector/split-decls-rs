macro_rules! deps {
    () => {
        Spawn!();
        MockTask!();
    };
}

macro_rules! spawn {
    () => {
        deps!();
        # [doc = " Spawn a future into a [`Spawn`] which wraps the future in a mocked executor."] # [doc = ""] # [doc = " This can be used to spawn a [`Future`] or a [`Stream`]."] # [doc = ""] # [doc = " For more information, check the module docs."] pub fn spawn < T > (task : T) -> Spawn < T > { Spawn { task : MockTask :: new () , future : Box :: pin (task) , } }
    };
}

spawn!()