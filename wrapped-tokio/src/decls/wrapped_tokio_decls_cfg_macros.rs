use serde::{Deserialize, Serialize};
use std::collections::HashMap;
cfg_macros! {
    #[doc = " Implementation detail of the `select!` macro. This macro is **not**"] #[doc
    = " intended to be used as part of the public API and is permitted to"] #[doc =
    " change."] #[doc(hidden)] pub use tokio_macros::select_priv_declare_output_enum;
    #[doc = " Implementation detail of the `select!` macro. This macro is **not**"] #[doc
    = " intended to be used as part of the public API and is permitted to"] #[doc =
    " change."] #[doc(hidden)] pub use tokio_macros::select_priv_clean_pattern; cfg_rt! {
    #[cfg(feature = "rt-multi-thread")] #[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
    #[doc(inline)] pub use tokio_macros::main; #[cfg(feature = "rt-multi-thread")]
    #[cfg_attr(docsrs, doc(cfg(feature = "macros")))] #[doc(inline)] pub use
    tokio_macros::test; cfg_not_rt_multi_thread! { #[doc(inline)] pub use
    tokio_macros::main_rt as main; #[doc(inline)] pub use tokio_macros::test_rt as test;
    } } cfg_not_rt! { #[doc(inline)] pub use tokio_macros::main_fail as main;
    #[doc(inline)] pub use tokio_macros::test_fail as test; }
}
