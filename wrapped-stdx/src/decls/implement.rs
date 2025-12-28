macro_rules! deps {
    () => {
        IntoBox!();
        Downcast!();
    };
}

macro_rules! implement {
    () => {
        deps!();
        macro_rules ! implement { ($ any_trait : ident $ (+ $ auto_traits : ident) *) => { impl Downcast for dyn $ any_trait $ (+ $ auto_traits) * { # [inline] fn type_id (& self) -> TypeId { self . type_id () } # [inline] unsafe fn downcast_unchecked_ref < T : 'static > (& self) -> & T { unsafe { &* std :: ptr :: from_ref ::< Self > (self) . cast ::< T > () } } # [inline] unsafe fn downcast_unchecked_mut < T : 'static > (& mut self) -> & mut T { unsafe { & mut * std :: ptr :: from_mut ::< Self > (self) . cast ::< T > () } } } impl < T : $ any_trait $ (+ $ auto_traits) *> IntoBox < dyn $ any_trait $ (+ $ auto_traits) *> for T { # [inline] fn into_box (self) -> Box < dyn $ any_trait $ (+ $ auto_traits) *> { Box :: new (self) } } } }
    };
}

implement!()