macro_rules! BarrierState {
    () => {
        struct BarrierState { count : usize , generation_id : usize , }
    };
}

BarrierState!();