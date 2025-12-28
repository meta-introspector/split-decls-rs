macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! custom_page_sz {
    () => {
        deps!();
        # [test] fn custom_page_sz () { let mut model = loom :: model :: Builder :: new () ; model . max_branches = 100000 ; model . check (| | { let slab = Slab :: < usize > :: new_with_config :: < TinyConfig > () ; for i in 0 .. 1024usize { test_println ! ("{}" , i) ; let k = slab . insert (i) . expect ("insert") ; let v = slab . get (k) . expect ("get") ; assert_eq ! (v , i , "slab: {:#?}" , slab) ; } }) ; }
    };
}

custom_page_sz!()