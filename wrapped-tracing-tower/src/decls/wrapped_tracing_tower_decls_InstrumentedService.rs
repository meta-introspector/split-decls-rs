use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub type InstrumentedService<S, R> = service_span::Service<request_span::Service<S, R>>;
