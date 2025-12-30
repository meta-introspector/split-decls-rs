// Generated macro for unstable (module)
macro_rules! Depcrateunstable {
() => {
// Module: crate
// Provides: {"unstable"}
// Dependencies: {}
# [doc = "Unstable APIs\n\
\
All APIs accessible by importing this module are unstable; They may be changed in patch \
releases. You MUST use an exact version specifier in `Cargo.toml`, to indicate the version of this \
API you're using:\n\
\
```toml\n
[dependencies]\n
zip = \"="] # [doc = env ! ("CARGO_PKG_VERSION")] # [doc = "\"\n\
```"] pub mod unstable ;
};
}
