macro_rules! shards {
    () => {
        # [inline] pub fn shards () -> usize { if is_dyn_thread_safe () { return SHARDS ; } 1 }
    };
}

shards!()