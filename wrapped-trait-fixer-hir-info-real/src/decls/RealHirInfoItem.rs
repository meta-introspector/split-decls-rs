macro_rules! RealHirInfoItem {
    () => {
        pub struct RealHirInfoItem < 'tcx > (pub Item < 'tcx >) ;
    };
}

RealHirInfoItem!()