macro_rules! Exfiltrator {
    () => {
        # [doc = " A trait describing what and how is extracted from signal handlers."] # [doc = ""] # [doc = " By choosing a specific implementor as the type parameter for"] # [doc = " [`SignalsInfo`][crate::iterator::SignalsInfo], one can pick how much and what information is"] # [doc = " returned from the iterator."] pub trait Exfiltrator : sealed :: Exfiltrator { }
    };
}

Exfiltrator!()