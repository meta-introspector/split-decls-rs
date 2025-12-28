macro_rules! deps {
    () => {
        End!();
        Cursor!();
        Entry!();
    };
}

macro_rules! start_of_buffer {
    () => {
        deps!();
        fn start_of_buffer (cursor : Cursor) -> * const Entry { unsafe { match & * cursor . scope { Entry :: End (offset , _) => cursor . scope . offset (* offset) , _ => unreachable ! () , } } }
    };
}

start_of_buffer!()