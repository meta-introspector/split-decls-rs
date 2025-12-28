macro_rules! PanicContext {
    () => {
        # [doc = " Dummy for leveraging RAII cleanup to pop frames."] # [must_use] pub struct PanicContext { _priv : () , }
    };
}

PanicContext!()