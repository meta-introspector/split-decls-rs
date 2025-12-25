use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[doc(hidden)]
pub mod __macro_support {
    pub use crate::callsite::Callsite;
    use crate::{subscriber::Interest, Metadata};
    pub use core::{concat, file, format_args, iter::Iterator, line, option::Option, stringify};
    use core::{fmt, str};
    /// Callsite implementation used by macro-generated code.
    ///
    /// /!\ WARNING: This is *not* a stable API! /!\
    /// This type, and all code contained in the `__macro_support` module, is
    /// a *private* API of `tracing`. It is exposed publicly because it is used
    /// by the `tracing` macros, but it is not part of the stable versioned API.
    /// Breaking changes to this module may occur in small-numbered versions
    /// without warning.
    pub use tracing_core::callsite::DefaultCallsite as MacroCallsite;
    /// /!\ WARNING: This is *not* a stable API! /!\
    /// This function, and all code contained in the `__macro_support` module, is
    /// a *private* API of `tracing`. It is exposed publicly because it is used
    /// by the `tracing` macros, but it is not part of the stable versioned API.
    /// Breaking changes to this module may occur in small-numbered versions
    /// without warning.
    pub fn __is_enabled(meta: &Metadata<'static>, interest: Interest) -> bool {
        interest.is_always() || crate::dispatcher::get_default(|default| default.enabled(meta))
    }
    /// /!\ WARNING: This is *not* a stable API! /!\
    /// This function, and all code contained in the `__macro_support` module, is
    /// a *private* API of `tracing`. It is exposed publicly because it is used
    /// by the `tracing` macros, but it is not part of the stable versioned API.
    /// Breaking changes to this module may occur in small-numbered versions
    /// without warning.
    #[inline]
    #[cfg(feature = "log")]
    pub fn __disabled_span(meta: &'static Metadata<'static>) -> crate::Span {
        crate::Span::new_disabled(meta)
    }
    /// /!\ WARNING: This is *not* a stable API! /!\
    /// This function, and all code contained in the `__macro_support` module, is
    /// a *private* API of `tracing`. It is exposed publicly because it is used
    /// by the `tracing` macros, but it is not part of the stable versioned API.
    /// Breaking changes to this module may occur in small-numbered versions
    /// without warning.
    #[inline]
    #[cfg(not(feature = "log"))]
    pub fn __disabled_span(_: &'static Metadata<'static>) -> crate::Span {
        crate::Span::none()
    }
    /// /!\ WARNING: This is *not* a stable API! /!\
    /// This function, and all code contained in the `__macro_support` module, is
    /// a *private* API of `tracing`. It is exposed publicly because it is used
    /// by the `tracing` macros, but it is not part of the stable versioned API.
    /// Breaking changes to this module may occur in small-numbered versions
    /// without warning.
    #[cfg(feature = "log")]
    pub fn __tracing_log(
        meta: &Metadata<'static>,
        logger: &'static dyn log::Log,
        log_meta: log::Metadata<'_>,
        values: &tracing_core::field::ValueSet<'_>,
    ) {
        logger.log(
            &crate::log::Record::builder()
                .file(meta.file())
                .module_path(meta.module_path())
                .line(meta.line())
                .metadata(log_meta)
                .args(format_args!(
                    "{}",
                    crate::log::LogValueSet {
                        values,
                        is_first: true
                    }
                ))
                .build(),
        );
    }
    /// Implementation detail used for constructing FieldSet names from raw
    /// identifiers. In `info!(..., r#type = "...")` the macro would end up
    /// constructing a name equivalent to `FieldName(*b"type")`.
    pub struct FieldName<const N: usize>([u8; N]);
    impl<const N: usize> FieldName<N> {
        /// Convert `"prefix.r#keyword.suffix"` to `b"prefix.keyword.suffix"`.
        pub const fn new(input: &str) -> Self {
            let input = input.as_bytes();
            let mut output = [0u8; N];
            let mut read = 0;
            let mut write = 0;
            while read < input.len() {
                if read + 1 < input.len() && input[read] == b'r' && input[read + 1] == b'#' {
                    read += 2;
                }
                output[write] = input[read];
                read += 1;
                write += 1;
            }
            assert!(write == N);
            Self(output)
        }
        pub const fn as_str(&self) -> &str {
            unsafe { str::from_utf8_unchecked(self.0.as_slice()) }
        }
    }
    impl FieldName<0> {
        /// For `"prefix.r#keyword.suffix"` compute `"prefix.keyword.suffix".len()`.
        pub const fn len(input: &str) -> usize {
            let mut raw = 0;
            let mut i = 0;
            while i < input.len() {
                if input.as_bytes()[i] == b'#' {
                    raw += 1;
                }
                i += 1;
            }
            input.len() - 2 * raw
        }
    }
    impl<const N: usize> fmt::Debug for FieldName<N> {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter
                .debug_tuple("FieldName")
                .field(&self.as_str())
                .finish()
        }
    }
}
