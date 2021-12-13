use crate::config::ServiceOptions;
use yansi::Paint;

pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

fn print_format(name: &str, value: &str) {
    println!(
        "=> {}: {}",
        Paint::green(name).bold(),
        Paint::default(value).bold()
    );
}

pub fn print_server_info(opts: ServiceOptions) {
    println!(
        "{}",
        Paint::default("North Configuration")
            .bold()
            .underline()
            .blink()
    );
    print_format("name", opts.name.unwrap().as_str());
    print_format("version", opts.version.unwrap().as_str());
    print_format("address", opts.address.unwrap().as_str());
    print_format("port", opts.port.unwrap().to_string().as_str());
    print_format("keep alive", opts.keep_alive.to_string().as_str());
    print_format(
        "read timeout",
        format!("{}{}", opts.read_timeout.to_string().as_str(), "s").as_str(),
    );
    print_format(
        "write timeout",
        format!("{}{}", opts.write_timeout.to_string().as_str(), "s").as_str(),
    );
    print_format(
        "write timeout",
        format!("{}{}", opts.write_timeout.to_string().as_str(), "s").as_str(),
    );
    if opts.graceful_shutdown {
        print_format("graceful shutdown", "enabled");
    } else {
        print_format("graceful shutdown", "disabled");
    }
}
