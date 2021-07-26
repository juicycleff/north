pub trait ServiceTrait {
    type Output;

    /// takes in a handler function
    fn handler(self) -> Self::Output;

    /// takes in the version of the service
    fn version(self, version: &str) -> Self::Output;

    /// takes in the name of the service
    fn name(self, name: &str) -> Self::Output;

    /// takes in the name of the service
    fn keep_alive(self, timeout: u32) -> Self::Output;

    /// takes in the name of the service
    fn read_timeout(self, timeout: u32) -> Self::Output;

    /// takes in the name of the service
    fn write_timeout(self, timeout: u32) -> Self::Output;

    /// takes in the name of the service
    fn address(self, address: &str) -> Self::Output;

    /// takes in the name of the service
    fn port(self, port: u16) -> Self::Output;

    fn wrapper(self) -> Self::Output;

    fn graceful_shutdown(self) -> Self::Output;

    fn middleware(self) -> Self::Output;
}
