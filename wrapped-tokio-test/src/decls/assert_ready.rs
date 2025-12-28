macro_rules! assert_ready {
    () => {
        # [doc = " Asserts a `Poll` is ready, returning the value."] # [doc = ""] # [doc = " This will invoke `panic!` if the provided `Poll` does not evaluate to `Poll::Ready` at"] # [doc = " runtime."] # [doc = ""] # [doc = " # Custom Messages"] # [doc = ""] # [doc = " This macro has a second form, where a custom panic message can be provided with or without"] # [doc = " arguments for formatting."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_util::future;"] # [doc = " use tokio_test::{assert_ready, task};"] # [doc = ""] # [doc = " let mut fut = task::spawn(future::ready(()));"] # [doc = " assert_ready!(fut.poll());"] # [doc = " ```"] # [macro_export] macro_rules ! assert_ready { ($ e : expr) => { { use core :: task :: Poll ; match $ e { Poll :: Ready (v) => v , Poll :: Pending => panic ! ("pending") , } } } ; ($ e : expr , $ ($ msg : tt) +) => { { use core :: task :: Poll ; match $ e { Poll :: Ready (v) => v , Poll :: Pending => { panic ! ("pending; {}" , format_args ! ($ ($ msg) +)) } } } } ; }
    };
}

assert_ready!()