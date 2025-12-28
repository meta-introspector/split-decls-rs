macro_rules! StateDiffCollector {
    () => {
        struct StateDiffCollector < D > { prev_state : D , before : Option < Vec < String > > , after : Vec < String > , }
    };
}

StateDiffCollector!();