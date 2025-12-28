macro_rules! deps {
    () => {
        Interner!();
    };
}

macro_rules! Features {
    () => {
        deps!();
        pub trait Features < I : Interner > : Copy { fn generic_const_exprs (self) -> bool ; fn coroutine_clone (self) -> bool ; fn associated_const_equality (self) -> bool ; fn feature_bound_holds_in_crate (self , symbol : I :: Symbol) -> bool ; }
    };
}

Features!()