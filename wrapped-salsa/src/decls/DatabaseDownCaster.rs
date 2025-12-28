macro_rules! deps {
    () => {
        ViewCaster!();
    };
}

macro_rules! DatabaseDownCaster {
    () => {
        deps!();
        # [repr (transparent)] pub struct DatabaseDownCaster < DbView : ? Sized > (ViewCaster , PhantomData < fn () -> DbView >) ;
    };
}

DatabaseDownCaster!()