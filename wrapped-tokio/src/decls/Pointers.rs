macro_rules! deps {
    () => {
        UnsafeCell!();
        PointersInner!();
    };
}

macro_rules! Pointers {
    () => {
        deps!();
        # [doc = " Previous / next pointers."] pub (crate) struct Pointers < T > { inner : UnsafeCell < PointersInner < T > > , }
    };
}

Pointers!()