macro_rules! fs_async_serial_core_with_return {
    () => {
        # [doc (hidden)] # [cfg (feature = "async")] pub async fn fs_async_serial_core_with_return < E > (names : Vec < & str > , path : Option < & str > , fut : impl std :: future :: Future < Output = Result < () , E > > ,) -> Result < () , E > { let mut locks = get_locks (& names , path) ; locks . iter_mut () . for_each (| lock | lock . start_serial ()) ; let ret : Result < () , E > = fut . await ; locks . into_iter () . for_each (| lock | lock . end_serial ()) ; ret }
    };
}

fs_async_serial_core_with_return!();