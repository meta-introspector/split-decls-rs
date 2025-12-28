macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_667 {
    () => {
        deps!();
        impl < T , P > IndexMut < usize > for Punctuated < T , P > { fn index_mut (& mut self , index : usize) -> & mut Self :: Output { if index . checked_add (1) == Some (self . len ()) { match & mut self . last { Some (t) => t , None => & mut self . inner [index] . 0 , } } else { & mut self . inner [index] . 0 } } }
    };
}

impl_667!();