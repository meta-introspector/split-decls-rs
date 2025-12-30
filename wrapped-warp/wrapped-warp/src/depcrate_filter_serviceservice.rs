// Generated macro for service (function)
macro_rules! Depcrate_filter_serviceservice {
() => {
// Module: crate::filter::service
// Provides: {"service"}
// Dependencies: {}
# [doc = " Convert a `Filter` into a `Service`."] # [doc = ""] # [doc = " Filters are normally what APIs are built on in warp. However, it can be"] # [doc = " useful to convert a `Filter` into a [`Service`][Service], such as if"] # [doc = " further customizing a `hyper::Service`, or if wanting to make use of"] # [doc = " the greater [Tower][tower] set of middleware."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Running a `warp::Filter` on a regular `hyper::Server`:"] # [doc = ""] # [doc = " ```"] # [doc = " # async fn run() -> Result<(), Box<dyn std::error::Error>> {"] # [doc = " use std::convert::Infallible;"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " // Our Filter..."] # [doc = " let route = warp::any().map(|| \"Hello From Warp!\");"] # [doc = ""] # [doc = " // Convert it into a `Service`..."] # [doc = " let svc = warp::service(route);"] # [doc = " # drop(svc);"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " [Service]: https://docs.rs/tower_service/latest/tower_service/trait.Service.html"] # [doc = " [tower]: https://docs.rs/tower"] pub fn service < F > (filter : F) -> FilteredService < F > where F : Filter , < F :: Future as TryFuture > :: Ok : Reply , < F :: Future as TryFuture > :: Error : IsReject , { FilteredService { filter } }
};
}
