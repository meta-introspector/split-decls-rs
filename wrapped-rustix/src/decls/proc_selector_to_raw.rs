macro_rules! deps {
    () => {
        RawPid!();
        IdType!();
        Pid!();
        ProcSelector!();
    };
}

macro_rules! proc_selector_to_raw {
    () => {
        deps!();
        fn proc_selector_to_raw (selector : ProcSelector) -> (IdType , RawPid) { match selector { Some ((idtype , id)) => (idtype , id . as_raw_nonzero () . get ()) , None => (IdType :: Pid , 0) , } }
    };
}

proc_selector_to_raw!();