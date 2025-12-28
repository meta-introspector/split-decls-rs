macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a , T , const N : usize > Iterator for Iter < 'a , T , N > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { let entry = self . cache . entries . get (self . pos as usize) ? ; self . pos = if self . pos == self . cache . tail { N as u16 } else { entry . next } ; Some (& entry . val) } }
    };
}

impl_8!()