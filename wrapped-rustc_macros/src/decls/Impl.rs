macro_rules! Impl {
    () => {
        struct Impl { attrs : Vec < Attribute > , generics : Generics , self_ty : Type , items : Vec < ImplItem > , wc : Option < WhereClause > , }
    };
}

Impl!()