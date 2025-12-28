macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! custom_panic_handler_and_spawn {
    () => {
        deps!();
        # [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn custom_panic_handler_and_spawn () { let (tx , rx) = channel () ; let tx = Mutex :: new (tx) ; let panic_handler = move | e : Box < dyn Any + Send > | { tx . lock () . unwrap () . send (e) . unwrap () ; } ; let builder = ThreadPoolBuilder :: new () . panic_handler (panic_handler) ; builder . build () . unwrap () . spawn (move | | { panic ! ("Hello, world!") ; }) ; let error = rx . recv () . unwrap () ; if let Some (& msg) = error . downcast_ref :: < & str > () { assert_eq ! (msg , "Hello, world!") ; } else { panic ! ("did not receive a string from panic handler") ; } }
    };
}

custom_panic_handler_and_spawn!()