// Generated macro for last_event_id (function)
macro_rules! Depcrate_filters_sselast_event_id {
() => {
// Module: crate::filters::sse
// Provides: {"last_event_id"}
// Dependencies: {}
# [doc = " Gets the optional last event id from request."] # [doc = " Typically this identifier represented as number or string."] # [doc = ""] # [doc = " ```"] # [doc = " let app = warp::sse::last_event_id::<u32>();"] # [doc = ""] # [doc = " // The identifier is present"] # [doc = " # #[cfg(feature = \"test\")]"] # [doc = " async {"] # [doc = "     assert_eq!("] # [doc = "         warp::test::request()"] # [doc = "            .header(\"Last-Event-ID\", \"12\")"] # [doc = "            .filter(&app)"] # [doc = "            .await"] # [doc = "            .unwrap(),"] # [doc = "         Some(12)"] # [doc = "     );"] # [doc = ""] # [doc = "     // The identifier is missing"] # [doc = "     assert_eq!("] # [doc = "        warp::test::request()"] # [doc = "            .filter(&app)"] # [doc = "            .await"] # [doc = "            .unwrap(),"] # [doc = "         None"] # [doc = "     );"] # [doc = ""] # [doc = "     // The identifier is not a valid"] # [doc = "     assert!("] # [doc = "        warp::test::request()"] # [doc = "            .header(\"Last-Event-ID\", \"abc\")"] # [doc = "            .filter(&app)"] # [doc = "            .await"] # [doc = "            .is_err(),"] # [doc = "     );"] # [doc = "};"] # [doc = " ```"] pub fn last_event_id < T > () -> impl Filter < Extract = One < Option < T > > , Error = Rejection > + Copy where T : FromStr + Send + Sync + 'static , { header :: optional ("last-event-id") }
};
}
