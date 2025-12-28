macro_rules! deps {
    () => {
        IntoIter!();
        Punctuated!();
    };
}

macro_rules! impl_612 {
    () => {
        deps!();
        impl < T , P > IntoIterator for Punctuated < T , P > { type Item = T ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { let mut elements = Vec :: with_capacity (self . len ()) ; for (t , _) in self . inner { elements . push (t) ; } if let Some (t) = self . last { elements . push (* t) ; } IntoIter { inner : elements . into_iter () , } } }
    };
}

impl_612!();