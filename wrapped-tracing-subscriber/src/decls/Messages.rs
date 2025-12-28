macro_rules! Messages {
    () => {
        # [doc = " A visitor wrapper that ensures any strings named \"message\" are formatted"] # [doc = " using `fmt::Display`"] # [derive (Debug , Clone)] pub struct Messages < V > (V) ;
    };
}

Messages!()