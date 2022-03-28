use std::convert::Infallible;

use futures::task::SpawnExt;
use hyper::service::{make_service_fn, service_fn};

use north_common::logger::init_logger;

use crate::config::ServiceOptions;
use crate::handler::handler_service;
use crate::serve;
use crate::service::Service;
use crate::utils::{print_server_info, shutdown_signal};

pub struct North {
    pub service: Service,
}

pub fn power() -> Service {
    init_logger();

    Service {
        options: ServiceOptions::default(),
    }
}

impl Service {
    pub fn up(self) -> North {
        North { service: self }
    }
}

impl North {
    pub async fn start(mut self) {
        let full_addr = format!(
            "{}:{}",
            self.service.options.address.clone().unwrap(),
            self.service.options.port.unwrap()
        );

        serve!(self, full_addr, async |builder, protocol| {
            let make_svc =
                make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handler_service)) });
            let server = builder.serve(make_svc);

            print_server_info(self.service.options.clone());

            if self.service.options.graceful_shutdown {
                let graceful = server.with_graceful_shutdown(shutdown_signal());
                log::info!("north served on {}{}", protocol, full_addr);
                if let Err(e) = graceful.await {
                    log::info!("server error: {}", e);
                }
                return;
            };

            log::info!("north served on {}{}", protocol, full_addr);
            if let Err(e) = server.await {
                log::info!("server error: {}", e);
            }
        });
        /* let sock_addr = full_addr
            .to_socket_addrs()
            .expect("Invalid server address")
            .next()
            .unwrap();
        let builder = hyper::Server::bind(&sock_addr);

        let make_svc =
            make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handler_service)) });

        let server = builder.serve(make_svc);
        if self.service.options.graceful_shutdown {
            let graceful = &server.with_graceful_shutdown(shutdown_signal());
            println!("server started");
            if let Err(e) = graceful.await {
                eprintln!("server error: {}", e);
            }
            return;
        };

        println!("server started");
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        } */
    }
}
