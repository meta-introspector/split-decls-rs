macro_rules! Empty {
    () => {
        # [doc = " Stream for the [`empty`](fn@empty) function."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Empty < T > (PhantomData < T >) ;
    };
}

Empty!();