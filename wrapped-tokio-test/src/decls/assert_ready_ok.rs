macro_rules! assert_ready_ok {
    () => {
        # [doc = " Asserts a `Poll<Result<...>>` is ready and `Ok`, returning the value."] # [doc = ""] # [doc = " This will invoke `panic!` if the provided `Poll` does not evaluate to `Poll::Ready(Ok(..))` at"] # [doc = " runtime."] # [doc = ""] # [doc = " # Custom Messages"] # [doc = ""] # [doc = " This macro has a second form, where a custom panic message can be provided with or without"] # [doc = " arguments for formatting."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_util::future;"] # [doc = " use tokio_test::{assert_ready_ok, task};"] # [doc = ""] # [doc = " let mut fut = task::spawn(future::ok::<_, ()>(()));"] # [doc = " assert_ready_ok!(fut.poll());"] # [doc = " ```"] # [macro_export] macro_rules ! assert_ready_ok { ($ e : expr) => { { use tokio_test :: { assert_ready , assert_ok } ; let val = assert_ready ! ($ e) ; assert_ok ! (val) } } ; ($ e : expr , $ ($ msg : tt) +) => { { use tokio_test :: { assert_ready , assert_ok } ; let val = assert_ready ! ($ e , $ ($ msg) *) ; assert_ok ! (val , $ ($ msg) *) } } ; }
    };
}

assert_ready_ok!();