macro_rules! deps {
    () => {
        Group!();
        Cursor!();
        Entry!();
    };
}

macro_rules! open_span_of_group {
    () => {
        deps!();
        pub (crate) fn open_span_of_group (cursor : Cursor) -> Span { match cursor . entry () { Entry :: Group (group , _) => group . span_open () , _ => cursor . span () , } }
    };
}

open_span_of_group!();