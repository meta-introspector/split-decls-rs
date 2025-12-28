macro_rules! deps {
    () => {
        State!();
        InflateStream!();
    };
}

macro_rules! end {
    () => {
        deps!();
        pub fn end < 'a > (stream : & 'a mut InflateStream < 'a >) -> & 'a mut z_stream { let alloc = stream . alloc ; let mut window = Window :: empty () ; core :: mem :: swap (& mut window , & mut stream . state . window) ; if ! window . is_empty () { let (ptr , len) = window . into_raw_parts () ; unsafe { alloc . deallocate (ptr , len) } ; } let stream = stream . as_z_stream_mut () ; let state_ptr = core :: mem :: replace (& mut stream . state , core :: ptr :: null_mut ()) ; unsafe { alloc . deallocate (state_ptr as * mut State , 1) } ; stream }
    };
}

end!()