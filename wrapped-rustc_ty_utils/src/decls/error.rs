macro_rules! error {
    () => {
        fn error < 'tcx > (cx : & LayoutCx < 'tcx > , err : LayoutError < 'tcx >) -> & 'tcx LayoutError < 'tcx > { cx . tcx () . arena . alloc (err) }
    };
}

error!();