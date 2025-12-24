use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/**
Stream a value through a stream with an arbitrarily short lifetime.
*/
pub fn stream_computed<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: impl Value,
) -> Result {
    stream.value_computed(&value)
}
