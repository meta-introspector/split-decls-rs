macro_rules! swap {
    () => {
        # [inline (always)] fn swap < T , U > ((t , u) : (T , U)) -> (U , T) { (u , t) }
    };
}

swap!();