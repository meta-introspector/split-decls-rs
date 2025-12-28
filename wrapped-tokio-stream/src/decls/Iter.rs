macro_rules! Iter {
    () => {
        # [doc = " Stream for the [`iter`](fn@iter) function."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Iter < I > { iter : I , yield_amt : usize , }
    };
}

Iter!();