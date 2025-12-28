macro_rules! PageDataEntry {
    () => {
        type PageDataEntry < T > = UnsafeCell < MaybeUninit < T > > ;
    };
}

PageDataEntry!()