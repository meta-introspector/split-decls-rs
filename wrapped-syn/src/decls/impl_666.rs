macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_666 {
    () => {
        deps!();
        impl < T , P > Index < usize > for Punctuated < T , P > { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { if index . checked_add (1) == Some (self . len ()) { match & self . last { Some (t) => t , None => & self . inner [index] . 0 , } } else { & self . inner [index] . 0 } } }
    };
}

impl_666!();