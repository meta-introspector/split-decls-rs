macro_rules! deps {
    () => {
        CycleHeads!();
    };
}

macro_rules! empty_cycle_heads {
    () => {
        deps!();
        # [inline] pub (crate) fn empty_cycle_heads () -> & 'static CycleHeads { static EMPTY_CYCLE_HEADS : OnceLock < CycleHeads > = OnceLock :: new () ; EMPTY_CYCLE_HEADS . get_or_init (| | CycleHeads (ThinVec :: new ())) }
    };
}

empty_cycle_heads!()