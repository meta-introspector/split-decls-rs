macro_rules! deps {
    () => {
        Default!();
        Field!();
        Variant!();
        Ctxt!();
    };
}

macro_rules! fields_from_ast {
    () => {
        deps!();
        fn fields_from_ast < 'a > (cx : & Ctxt , fields : & 'a Punctuated < syn :: Field , Token ! [,] > , attrs : Option < & attr :: Variant > , container_default : & attr :: Default , private : & Ident ,) -> Vec < Field < 'a > > { fields . iter () . enumerate () . map (| (i , field) | Field { member : match & field . ident { Some (ident) => syn :: Member :: Named (ident . clone ()) , None => syn :: Member :: Unnamed (i . into ()) , } , attrs : attr :: Field :: from_ast (cx , i , field , attrs , container_default , private) , ty : & field . ty , original : field , }) . collect () }
    };
}

fields_from_ast!()