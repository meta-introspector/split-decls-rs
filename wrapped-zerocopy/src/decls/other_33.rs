macro_rules! deps {
    () => {
        AlignOf!();
    };
}

macro_rules! other_33 {
    () => {
        deps!();
        # [doc = " A type whose size is equal to `max(align_of::<T>(), align_of::<U>())`."] # [repr (C)] pub union MaxAlignsOf < T , U > { _t : ManuallyDrop < AlignOf < T > > , _u : ManuallyDrop < AlignOf < U > > , }
    };
}

other_33!()