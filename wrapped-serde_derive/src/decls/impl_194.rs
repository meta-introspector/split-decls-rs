macro_rules! deps {
    () => {
        BorrowedLifetimes!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl BorrowedLifetimes { fn de_lifetime (& self) -> syn :: Lifetime { match * self { BorrowedLifetimes :: Borrowed (_) => syn :: Lifetime :: new ("'de" , Span :: call_site ()) , BorrowedLifetimes :: Static => syn :: Lifetime :: new ("'static" , Span :: call_site ()) , } } fn de_lifetime_param (& self) -> Option < syn :: LifetimeParam > { match self { BorrowedLifetimes :: Borrowed (bounds) => Some (syn :: LifetimeParam { attrs : Vec :: new () , lifetime : syn :: Lifetime :: new ("'de" , Span :: call_site ()) , colon_token : None , bounds : bounds . iter () . cloned () . collect () , }) , BorrowedLifetimes :: Static => None , } } }
    };
}

impl_194!();