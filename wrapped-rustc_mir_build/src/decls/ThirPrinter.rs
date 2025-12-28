macro_rules! ThirPrinter {
    () => {
        struct ThirPrinter < 'a , 'tcx > { thir : & 'a Thir < 'tcx > , fmt : String , }
    };
}

ThirPrinter!()