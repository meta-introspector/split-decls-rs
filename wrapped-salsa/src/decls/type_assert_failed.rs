macro_rules! deps {
    () => {
        Page!();
    };
}

macro_rules! type_assert_failed {
    () => {
        deps!();
        # [doc = " This function is explicitly outlined to avoid debug machinery in the hot-path."] # [cold] # [inline (never)] fn type_assert_failed < T : 'static > (page : & Page) -> ! { panic ! ("page has slot type `{:?}` but `{:?}` was expected" , (page . slot_vtable . type_name) () , std :: any :: type_name ::< T > () ,) }
    };
}

type_assert_failed!();