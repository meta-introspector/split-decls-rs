macro_rules! WeakArrayMut {
    () => {
        # [derive (Debug)] pub (crate) struct WeakArrayMut < 'a , T , const N : usize > { ptr : * mut [T ; N] , _marker : PhantomData < & 'a mut [T ; N] > , }
    };
}

WeakArrayMut!();