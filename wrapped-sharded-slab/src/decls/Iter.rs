macro_rules! deps {
    () => {
        Slot!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        pub (crate) type Iter < 'a , T , C > = std :: iter :: FilterMap < std :: slice :: Iter < 'a , Slot < Option < T > , C > > , fn (& 'a Slot < Option < T > , C >) -> Option < & 'a T > , > ;
    };
}

Iter!()