macro_rules! deps {
    () => {
        Container!();
    };
}

macro_rules! type_of_item {
    () => {
        deps!();
        fn type_of_item (cont : & Container) -> syn :: Type { syn :: Type :: Path (syn :: TypePath { qself : None , path : syn :: Path { leading_colon : None , segments : vec ! [syn :: PathSegment { ident : cont . ident . clone () , arguments : syn :: PathArguments :: AngleBracketed (syn :: AngleBracketedGenericArguments { colon2_token : None , lt_token : < Token ! [<] >:: default () , args : cont . generics . params . iter () . map (| param | match param { syn :: GenericParam :: Type (param) => { syn :: GenericArgument :: Type (syn :: Type :: Path (syn :: TypePath { qself : None , path : param . ident . clone () . into () , })) } syn :: GenericParam :: Lifetime (param) => { syn :: GenericArgument :: Lifetime (param . lifetime . clone ()) } syn :: GenericParam :: Const (_) => { panic ! ("Serde does not support const generics yet") ; } }) . collect () , gt_token : < Token ! [>] >:: default () , } ,) , }] . into_iter () . collect () , } , }) }
    };
}

type_of_item!();