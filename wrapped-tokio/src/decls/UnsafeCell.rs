macro_rules! UnsafeCell {
    () => {
        # [derive (Debug)] pub (crate) struct UnsafeCell < T > (std :: cell :: UnsafeCell < T >) ;
    };
}

UnsafeCell!();