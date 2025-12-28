macro_rules! deps {
    () => {
        LinkedList!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl < L , T > LinkedList < L , T > { # [doc = " Creates an empty linked list."] pub (crate) const fn new () -> LinkedList < L , T > { LinkedList { head : None , tail : None , _marker : PhantomData , } } }
    };
}

impl_303!();