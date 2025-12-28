macro_rules! Mutex {
    () => {
        # [doc = " This wraps `spin::Mutex` to return a `Result`, so that it can be"] # [doc = " used with code written against `std::sync::Mutex`."] # [doc = ""] # [doc = " Since `spin::Mutex` doesn't support poisoning, the `Result` returned"] # [doc = " by `lock` will always be `Ok`."] # [derive (Debug , Default)] pub (crate) struct Mutex < T > { inner : crate :: spin :: Mutex < T > , }
    };
}

Mutex!();