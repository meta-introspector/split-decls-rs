macro_rules! check {
    () => {
        fn check < D : Default + PartialEq > (result : D) -> D { if result == D :: default () { panic ! ("allocation failed") ; } result }
    };
}

check!()