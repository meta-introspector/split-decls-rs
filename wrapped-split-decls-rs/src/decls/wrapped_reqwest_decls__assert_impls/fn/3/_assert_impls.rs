use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn _assert_impls () { fn assert_send < T : Send > () { } fn assert_sync < T : Sync > () { } fn assert_clone < T : Clone > () { } assert_send :: < Client > () ; assert_sync :: < Client > () ; assert_clone :: < Client > () ; assert_send :: < Request > () ; assert_send :: < RequestBuilder > () ; # [cfg (not (target_arch = "wasm32"))] { assert_send :: < Response > () ; } assert_send :: < Error > () ; assert_sync :: < Error > () ; assert_send :: < Body > () ; assert_sync :: < Body > () ; }
}