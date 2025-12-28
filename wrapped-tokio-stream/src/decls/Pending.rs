macro_rules! Pending {
    () => {
        # [doc = " Stream for the [`pending`](fn@pending) function."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Pending < T > (PhantomData < T >) ;
    };
}

Pending!();