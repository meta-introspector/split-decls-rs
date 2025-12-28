macro_rules! deps {
    () => {
        End!();
        Pair!();
        Punctuated!();
    };
}

macro_rules! do_extend {
    () => {
        deps!();
        fn do_extend < T , P , I > (punctuated : & mut Punctuated < T , P > , i : I) where I : Iterator < Item = Pair < T , P > > , { let mut nomore = false ; for pair in i { if nomore { panic ! ("punctuated extended with items after a Pair::End") ; } match pair { Pair :: Punctuated (a , b) => punctuated . inner . push ((a , b)) , Pair :: End (a) => { punctuated . last = Some (Box :: new (a)) ; nomore = true ; } } } }
    };
}

do_extend!();