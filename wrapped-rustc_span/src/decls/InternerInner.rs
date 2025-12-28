macro_rules! InternerInner {
    () => {
        struct InternerInner { arena : DroplessArena , byte_strs : FxIndexSet < & 'static [u8] > , }
    };
}

InternerInner!()