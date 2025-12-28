macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! Once {
    () => {
        deps!();
        # [doc = " Stream for the [`once`](fn@once) function."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Once < T > { iter : Iter < option :: IntoIter < T > > , }
    };
}

Once!()