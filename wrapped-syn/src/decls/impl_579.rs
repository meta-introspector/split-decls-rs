macro_rules! impl_579 {
    () => {
        impl PathArguments { pub fn is_empty (& self) -> bool { match self { PathArguments :: None => true , PathArguments :: AngleBracketed (bracketed) => bracketed . args . is_empty () , PathArguments :: Parenthesized (_) => false , } } pub fn is_none (& self) -> bool { match self { PathArguments :: None => true , PathArguments :: AngleBracketed (_) | PathArguments :: Parenthesized (_) => false , } } }
    };
}

impl_579!()