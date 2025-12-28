macro_rules! panic_propagate_nested_scope_spawn {
    () => {
        # [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_nested_scope_spawn () { scope (| s | s . spawn (| _ | scope (| s | s . spawn (| _ | panic ! ("Hello, world!"))))) ; }
    };
}

panic_propagate_nested_scope_spawn!();