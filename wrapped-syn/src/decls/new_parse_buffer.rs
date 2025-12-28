macro_rules! deps {
    () => {
        Cursor!();
        ParseBuffer!();
        Unexpected!();
    };
}

macro_rules! new_parse_buffer {
    () => {
        deps!();
        pub (crate) fn new_parse_buffer (scope : Span , cursor : Cursor , unexpected : Rc < Cell < Unexpected > > ,) -> ParseBuffer { ParseBuffer { scope , cell : Cell :: new (unsafe { mem :: transmute :: < Cursor , Cursor < 'static > > (cursor) }) , marker : PhantomData , unexpected : Cell :: new (Some (unexpected)) , } }
    };
}

new_parse_buffer!()