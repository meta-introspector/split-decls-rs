use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<
        T: tracing::Subscriber
            + for<'span> tracing_subscriber::registry::LookupSpan<'span>
            + Send
            + Sync,
    > BuildSubscriberRet for T
{
}
