// Generated macro for async_with_vars (function)
macro_rules! Depcrateasync_with_vars {
() => {
// Module: crate
// Provides: {"async_with_vars"}
// Dependencies: {}
# [cfg (feature = "async_closure")] # [doc = " Does the same as [`with_vars`] but it allows to pass an async closures."] # [doc = ""] # [doc = " ```rust"] # [doc = " async fn check_var() {"] # [doc = "     let v = std::env::var(\"MY_VAR\").unwrap();"] # [doc = "     assert_eq!(v, \"ok\".to_owned());"] # [doc = " }"] # [doc = ""] # [doc = " #[cfg(feature = \"async_closure\")]"] # [doc = " #[tokio::test]"] # [doc = " async fn test_async_closure() {"] # [doc = "     crate::async_with_vars([(\"MY_VAR\", Some(\"ok\"))], check_var());"] # [doc = " }"] # [doc = " ```"] pub async fn async_with_vars < K , V , F , R > (kvs : impl AsRef < [(K , Option < V >)] > , closure : F) -> R where K : AsRef < OsStr > + Clone + Eq + Hash , V : AsRef < OsStr > + Clone , F : std :: future :: Future < Output = R > + std :: future :: IntoFuture < Output = R > , { let old_env = RestoreEnv :: capture (SERIAL_TEST . lock () , kvs . as_ref () . iter () . map (| (k , _) | k . as_ref ()) ,) ; for (key , value) in kvs . as_ref () { update_env (key , value . as_ref ()) ; } let retval = closure . await ; drop (old_env) ; retval }
};
}
