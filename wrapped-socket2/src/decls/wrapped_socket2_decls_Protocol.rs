use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Protocol specification used for creating sockets via `Socket::new`.
///
/// This is a newtype wrapper around an integer which provides a nicer API in
/// addition to an injection point for documentation.
///
/// This type is freely interconvertible with C's `int` type, however, if a raw
/// value needs to be provided.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Protocol(c_int);
