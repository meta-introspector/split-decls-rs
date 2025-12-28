macro_rules! input_tys {
    () => {
        pub fn input_tys (sig : & syn :: Signature , skip : usize) -> syn :: Result < Vec < & syn :: Type > > { sig . inputs . iter () . skip (skip) . map (| input | { if let syn :: FnArg :: Typed (typed) = input { Ok (& * typed . ty) } else { Err (syn :: Error :: new_spanned (input , "unexpected receiver")) } }) . collect () }
    };
}

input_tys!()