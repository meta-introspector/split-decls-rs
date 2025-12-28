macro_rules! assert_ready_err {
    () => {
        # [doc = " Asserts a `Poll<Result<...>>` is ready and `Err`, returning the error."] # [doc = ""] # [doc = " This will invoke `panic!` if the provided `Poll` does not evaluate to `Poll::Ready(Err(..))` at"] # [doc = " runtime."] # [doc = ""] # [doc = " # Custom Messages"] # [doc = ""] # [doc = " This macro has a second form, where a custom panic message can be provided with or without"] # [doc = " arguments for formatting."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_util::future;"] # [doc = " use tokio_test::{assert_ready_err, task};"] # [doc = ""] # [doc = " let mut fut = task::spawn(future::err::<(), _>(()));"] # [doc = " assert_ready_err!(fut.poll());"] # [doc = " ```"] # [macro_export] macro_rules ! assert_ready_err { ($ e : expr) => { { use tokio_test :: { assert_ready , assert_err } ; let val = assert_ready ! ($ e) ; assert_err ! (val) } } ; ($ e : expr , $ ($ msg : tt) +) => { { use tokio_test :: { assert_ready , assert_err } ; let val = assert_ready ! ($ e , $ ($ msg) *) ; assert_err ! (val , $ ($ msg) *) } } ; }
    };
}

assert_ready_err!();