use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An asynchronous [`Stream`] of arriving signals.
///
/// The stream doesn't return the signals in the order they were recieved by
/// the process and may merge signals received multiple times.
pub struct SignalsInfo<E: Exfiltrator = SignalOnly>(OwningSignalIterator<UnixStream, E>);
