// Generated macro for other_89 (other)
macro_rules! Depcrate_network_reachabilityother_89 {
() => {
// Module: crate::network_reachability
// Provides: {"other_89"}
// Dependencies: {}
extern "C" { pub fn SCNetworkReachabilityCreateWithAddress (allocator : CFAllocatorRef , address : * const sockaddr ,) -> SCNetworkReachabilityRef ; pub fn SCNetworkReachabilityCreateWithAddressPair (allocator : CFAllocatorRef , localAddress : * const sockaddr , remoteAddress : * const sockaddr ,) -> SCNetworkReachabilityRef ; pub fn SCNetworkReachabilityCreateWithName (allocator : CFAllocatorRef , nodename : * const :: core :: ffi :: c_char ,) -> SCNetworkReachabilityRef ; pub fn SCNetworkReachabilityGetTypeID () -> CFTypeID ; pub fn SCNetworkReachabilityGetFlags (target : SCNetworkReachabilityRef , flags : * mut SCNetworkReachabilityFlags ,) -> Boolean ; pub fn SCNetworkReachabilitySetCallback (target : SCNetworkReachabilityRef , callout : SCNetworkReachabilityCallBack , context : * mut SCNetworkReachabilityContext ,) -> Boolean ; pub fn SCNetworkReachabilityScheduleWithRunLoop (target : SCNetworkReachabilityRef , runLoop : CFRunLoopRef , runLoopMode : CFStringRef ,) -> Boolean ; pub fn SCNetworkReachabilityUnscheduleFromRunLoop (target : SCNetworkReachabilityRef , runLoop : CFRunLoopRef , runLoopMode : CFStringRef ,) -> Boolean ; pub fn SCNetworkReachabilitySetDispatchQueue (target : SCNetworkReachabilityRef , queue : dispatch_queue_t ,) -> Boolean ; }
};
}
