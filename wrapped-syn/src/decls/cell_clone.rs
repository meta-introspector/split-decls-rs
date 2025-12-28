macro_rules! cell_clone {
    () => {
        fn cell_clone < T : Default + Clone > (cell : & Cell < T >) -> T { let prev = cell . take () ; let ret = prev . clone () ; cell . set (prev) ; ret }
    };
}

cell_clone!();