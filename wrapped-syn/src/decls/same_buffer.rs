macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! same_buffer {
    () => {
        deps!();
        pub (crate) fn same_buffer (a : Cursor , b : Cursor) -> bool { ptr :: eq (start_of_buffer (a) , start_of_buffer (b)) }
    };
}

same_buffer!();