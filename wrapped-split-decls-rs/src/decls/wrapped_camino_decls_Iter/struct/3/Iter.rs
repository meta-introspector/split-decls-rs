use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An iterator over the [`Utf8Component`]s of a [`Utf8Path`], as [`str`] slices."] # [doc = ""] # [doc = " This `struct` is created by the [`iter`] method on [`Utf8Path`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`iter`]: Utf8Path::iter"] # [derive (Clone)] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct Iter < 'a > { inner : Utf8Components < 'a > , }