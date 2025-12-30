// Generated macro for PeakEwma (struct)
macro_rules! Depcrate_load_peak_ewmaPeakEwma {
() => {
// Module: crate::load::peak_ewma
// Provides: {"PeakEwma"}
// Dependencies: {}
# [doc = " Measures the load of the underlying service using Peak-EWMA load measurement."] # [doc = ""] # [doc = " [`PeakEwma`] implements [`Load`] with the [`Cost`] metric that estimates the amount of"] # [doc = " pending work to an endpoint. Work is calculated by multiplying the"] # [doc = " exponentially-weighted moving average (EWMA) of response latencies by the number of"] # [doc = " pending requests. The Peak-EWMA algorithm is designed to be especially sensitive to"] # [doc = " worst-case latencies. Over time, the peak latency value decays towards the moving"] # [doc = " average of latencies to the endpoint."] # [doc = ""] # [doc = " When no latency information has been measured for an endpoint, an arbitrary default"] # [doc = " RTT of 1 second is used to prevent the endpoint from being overloaded before a"] # [doc = " meaningful baseline can be established.."] # [doc = ""] # [doc = " ## Note"] # [doc = ""] # [doc = " This is derived from [Finagle][finagle], which is distributed under the Apache V2"] # [doc = " license. Copyright 2017, Twitter Inc."] # [doc = ""] # [doc = " [finagle]:"] # [doc = " https://github.com/twitter/finagle/blob/9cc08d15216497bb03a1cafda96b7266cfbbcff1/finagle-core/src/main/scala/com/twitter/finagle/loadbalancer/PeakEwma.scala"] # [derive (Debug)] pub struct PeakEwma < S , C = CompleteOnResponse > { service : S , decay_ns : f64 , rtt_estimate : Arc < Mutex < RttEstimate > > , completion : C , }
};
}
