macro_rules! deps {
    () => {
        Visit!();
    };
}

macro_rules! empty_visit {
    () => {
        deps!();
        macro_rules ! empty_visit { ($ name : ident , $ t : ty) => { fn $ name <'doc , V > (_v : & mut V , _node : &'doc $ t) where V : Visit <'doc > + ? Sized , { } } ; }
    };
}

empty_visit!()