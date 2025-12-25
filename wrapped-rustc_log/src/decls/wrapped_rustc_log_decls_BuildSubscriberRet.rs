use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait alias for the complex return type of `build_subscriber` in
/// [init_logger_with_additional_layer]. A [Registry] with any composition of [tracing::Subscriber]s
/// (e.g. `Registry::default().with(custom_layer)`) should be compatible with this type.
/// Having an alias is also useful so rustc_driver_impl does not need to explicitly depend on
/// `tracing_subscriber`.
pub trait BuildSubscriberRet: tracing::Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<
        'span,
    > + Send + Sync {}
