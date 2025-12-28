macro_rules! deps {
    () => {
        DontDropMe!();
        Pool!();
    };
}

macro_rules! dont_drop {
    () => {
        deps!();
        # [test] fn dont_drop () { run_model ("dont_drop" , | | { let pool : Pool < DontDropMe > = Pool :: new () ; let (item1 , value) = DontDropMe :: new (1) ; test_println ! ("-> dont_drop: Inserting into pool {}" , item1 . id) ; let idx = pool . create_with (move | item | * item = value) . expect ("create_with") ; item1 . assert_not_clear () ; test_println ! ("-> dont_drop: clearing idx: {}" , idx) ; pool . clear (idx) ; item1 . assert_clear () ; }) ; }
    };
}

dont_drop!();