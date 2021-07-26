use crate::config::ServiceOptions;
use north_common::ServiceTrait;

#[derive(Default, Clone)]
pub struct Service {
    pub options: ServiceOptions,
}

/// implement service trait for north service
impl ServiceTrait for Service {
    type Output = Service;

    fn handler(mut self) -> Self::Output {
        todo!()
    }

    fn version(mut self, version: &str) -> Self::Output {
        self.options.name = Some(version.to_string());
        self
    }

    fn name(mut self, name: &str) -> Self::Output {
        self.options.name = Some(name.to_string());
        self
    }

    fn keep_alive(mut self, timeout: u32) -> Self::Output {
        self.options.keep_alive = timeout;
        self
    }

    fn read_timeout(mut self, timeout: u32) -> Self::Output {
        self.options.read_timeout = timeout;
        self
    }

    fn write_timeout(mut self, timeout: u32) -> Self::Output {
        self.options.write_timeout = timeout;
        self
    }

    fn address(mut self, address: &str) -> Self::Output {
        self.options.address = Some(address.to_string());
        self
    }

    fn port(mut self, port: u16) -> Self::Output {
        self.options.port = Some(port);
        self
    }

    fn wrapper(mut self) -> Self::Output {
        todo!()
    }

    fn graceful_shutdown(mut self) -> Self::Output {
        self.options.graceful_shutdown = true;
        self
    }

    fn middleware(mut self) -> Self::Output {
        todo!()
    }
}
