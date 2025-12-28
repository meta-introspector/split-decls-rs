macro_rules! StringLayer {
    () => {
        # [doc = " A layer that holds a string."] # [doc = ""] # [doc = " Used to test that pointers returned by downcasting are actually valid."] struct StringLayer (& 'static str) ;
    };
}

StringLayer!()