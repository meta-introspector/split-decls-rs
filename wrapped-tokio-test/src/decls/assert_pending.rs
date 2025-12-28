macro_rules! assert_pending {
    () => {
        # [doc = " Asserts a `Poll` is pending."] # [doc = ""] # [doc = " This will invoke `panic!` if the provided `Poll` does not evaluate to `Poll::Pending` at"] # [doc = " runtime."] # [doc = ""] # [doc = " # Custom Messages"] # [doc = ""] # [doc = " This macro has a second form, where a custom panic message can be provided with or without"] # [doc = " arguments for formatting."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_util::future;"] # [doc = " use tokio_test::{assert_pending, task};"] # [doc = ""] # [doc = " let mut fut = task::spawn(future::pending::<()>());"] # [doc = " assert_pending!(fut.poll());"] # [doc = " ```"] # [macro_export] macro_rules ! assert_pending { ($ e : expr) => { { use core :: task :: Poll ; match $ e { Poll :: Pending => { } Poll :: Ready (v) => panic ! ("ready; value = {:?}" , v) , } } } ; ($ e : expr , $ ($ msg : tt) +) => { { use core :: task :: Poll ; match $ e { Poll :: Pending => { } Poll :: Ready (v) => { panic ! ("ready; value = {:?}; {}" , v , format_args ! ($ ($ msg) +)) } } } } ; }
    };
}

assert_pending!();