macro_rules! deps {
    () => {
        VisitMut!();
    };
}

macro_rules! empty_visit_mut {
    () => {
        deps!();
        macro_rules ! empty_visit_mut { ($ name : ident , $ t : ty) => { fn $ name < V > (_v : & mut V , _node : & mut $ t) where V : VisitMut + ? Sized , { } } ; }
    };
}

empty_visit_mut!()