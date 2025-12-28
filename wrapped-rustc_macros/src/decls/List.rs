macro_rules! List {
    () => {
        # [doc = " A type used to greedily parse another type until the input is empty."] struct List < T > (Vec < T >) ;
    };
}

List!()