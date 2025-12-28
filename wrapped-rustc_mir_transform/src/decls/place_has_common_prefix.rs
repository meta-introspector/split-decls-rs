macro_rules! place_has_common_prefix {
    () => {
        fn place_has_common_prefix < 'tcx > (left : & Place < 'tcx > , right : & Place < 'tcx >) -> bool { left . local == right . local && left . projection . iter () . zip (right . projection) . all (| (left , right) | left == right) }
    };
}

place_has_common_prefix!()