macro_rules! deps {
    () => {
        Unexpected!();
        ParseBuffer!();
    };
}

macro_rules! inner_unexpected {
    () => {
        deps!();
        fn inner_unexpected (buffer : & ParseBuffer) -> (Rc < Cell < Unexpected > > , Option < (Span , Delimiter) >) { let mut unexpected = get_unexpected (buffer) ; loop { match cell_clone (& unexpected) { Unexpected :: None => return (unexpected , None) , Unexpected :: Some (span , delimiter) => return (unexpected , Some ((span , delimiter))) , Unexpected :: Chain (next) => unexpected = next , } } }
    };
}

inner_unexpected!()