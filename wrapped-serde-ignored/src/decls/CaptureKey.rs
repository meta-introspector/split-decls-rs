macro_rules! CaptureKey {
    () => {
        # [doc = " Seed that saves the string into the given optional during `visit_str` and"] # [doc = " `visit_string`."] struct CaptureKey < 'a , X > { delegate : X , key : & 'a mut Option < String > , }
    };
}

CaptureKey!()