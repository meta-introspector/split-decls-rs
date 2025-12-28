macro_rules! deps {
    () => {
        PointersInner!();
        UnsafeCell!();
    };
}

macro_rules! Pointers {
    () => {
        deps!();
        # [doc = " Previous / next pointers."] pub (crate) struct Pointers < T > { inner : UnsafeCell < PointersInner < T > > , }
    };
}

Pointers!();