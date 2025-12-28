macro_rules! deps {
    () => {
        Dispatch!();
        Subscriber!();
        Kind!();
    };
}

macro_rules! WeakDispatch {
    () => {
        deps!();
        # [doc = " `WeakDispatch` is a version of [`Dispatch`] that holds a non-owning reference"] # [doc = " to a [`Subscriber`]."] # [doc = ""] # [doc = " The `Subscriber` may be accessed by calling [`WeakDispatch::upgrade`],"] # [doc = " which returns an `Option<Dispatch>`. If all [`Dispatch`] clones that point"] # [doc = " at the `Subscriber` have been dropped, [`WeakDispatch::upgrade`] will return"] # [doc = " `None`. Otherwise, it will return `Some(Dispatch)`."] # [doc = ""] # [doc = " A `WeakDispatch` may be created from a [`Dispatch`] by calling the"] # [doc = " [`Dispatch::downgrade`] method. The primary use for creating a"] # [doc = " [`WeakDispatch`] is to allow a Subscriber` to hold a cyclical reference to"] # [doc = " itself without creating a memory leak. See [here] for details."] # [doc = ""] # [doc = " This type is analogous to the [`std::sync::Weak`] type, but for a"] # [doc = " [`Dispatch`] rather than an [`Arc`]."] # [doc = ""] # [doc = " [`Arc`]: std::sync::Arc"] # [doc = " [here]: Subscriber#avoiding-memory-leaks"] # [derive (Clone)] pub struct WeakDispatch { subscriber : Kind < Weak < dyn Subscriber + Send + Sync > > , }
    };
}

WeakDispatch!()