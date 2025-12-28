use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " A type that can be used with turbofish syntax in [`Extend::extend`]."] # [doc = ""] # [doc = " It is unlikely that you will want to use this trait directly. You are probably looking for the"] # [doc = " [`Extend`] trait."] pub trait ExtendTarget < T > : sealed :: ExtendTargetSealed < T > { }
}