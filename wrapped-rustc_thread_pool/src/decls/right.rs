macro_rules! right {
    () => {
        # [doc = " ```compile_fail,E0277\n\nuse std::rc::Rc;\n\nrustc_thread_pool::join(|| (), || Rc::new(23)); //~ ERROR\n\n``` "] mod right { }
    };
}

right!()