macro_rules! const_assert_lt {
    () => {
        # [doc = " Asserts that constants are less than each other."] # [macro_export (local_inner_macros)] macro_rules ! const_assert_lt { ($ x : expr , $ ($ y : expr) ,+ $ (,) ?) => { const_assert_lt ! (@ build $ x , $ ($ y) ,+) ; } ; (@ build $ x : expr) => { } ; (@ build $ x : expr , $ ($ y : expr) ,+) => { const_assert ! ($ x < _head ! ($ ($ y) ,+)) ; const_assert_lt ! (@ build $ ($ y) ,+) ; } ; }
    };
}

const_assert_lt!()