macro_rules! deps {
    () => {
        Page!();
        Slot!();
    };
}

macro_rules! PageView {
    () => {
        deps!();
        # [doc = " A typed [`Page`] view."] pub (crate) struct PageView < 'p , T : Slot > (& 'p Page , PhantomData < & 'p T >) ;
    };
}

PageView!()