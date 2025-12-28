macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! fold {
    () => {
        deps!();
        # [cfg (all (feature = "fold" , any (feature = "full" , feature = "derive")))] pub (crate) fn fold < T , P , V , F > (punctuated : Punctuated < T , P > , fold : & mut V , mut f : F ,) -> Punctuated < T , P > where V : ? Sized , F : FnMut (& mut V , T) -> T , { let Punctuated { inner , last } = punctuated ; let mut inner = VecDeque :: from (inner) ; for _ in 0 .. inner . len () { if let Some ((t , p)) = inner . pop_front () { inner . push_back ((f (fold , t) , p)) ; } } Punctuated { inner : Vec :: from (inner) , last : match last { Some (t) => Some (Box :: new (f (fold , * t))) , None => None , } , } }
    };
}

fold!();