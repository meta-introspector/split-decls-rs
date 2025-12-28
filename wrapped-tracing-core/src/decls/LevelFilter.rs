macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! LevelFilter {
    () => {
        deps!();
        # [doc = " A filter comparable to a verbosity [`Level`]."] # [doc = ""] # [doc = " If a [`Level`] is considered less than or equal to a `LevelFilter`, it"] # [doc = " should be considered enabled; if greater than the `LevelFilter`, that level"] # [doc = " is disabled. See [`LevelFilter::current`] for more details."] # [doc = ""] # [doc = " Note that this is essentially identical to the `Level` type, but with the"] # [doc = " addition of an [`OFF`] level that completely disables all trace"] # [doc = " instrumentation."] # [doc = ""] # [doc = " See the documentation for the [`Level`] type to see how `Level`s"] # [doc = " and `LevelFilter`s interact."] # [doc = ""] # [doc = " [`OFF`]: LevelFilter::OFF"] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash)] pub struct LevelFilter (Option < Level >) ;
    };
}

LevelFilter!()