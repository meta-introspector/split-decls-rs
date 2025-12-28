macro_rules! deps {
    () => {
        PageDataEntry!();
    };
}

macro_rules! PageData {
    () => {
        deps!();
        type PageData < T > = [PageDataEntry < T > ; PAGE_LEN] ;
    };
}

PageData!();