macro_rules! run_with_tcx {
    () => {
        # [doc = " Instantiate and run the compiler with the provided arguments and callback."] # [doc = ""] # [doc = " This is similar to `run` but it invokes the callback with the compiler's `TyCtxt`,"] # [doc = " which can be used to invoke internal APIs."] # [macro_export] macro_rules ! run_with_tcx { ($ args : expr , $ callback_fn : ident) => { $ crate :: run_driver ! ($ args , | tcx | $ callback_fn (tcx) , with_tcx) } ; ($ args : expr , $ callback : expr) => { $ crate :: run_driver ! ($ args , $ callback , with_tcx) } ; }
    };
}

run_with_tcx!()