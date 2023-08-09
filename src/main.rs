use flexi_logger::{Cleanup, Criterion, Naming};
use log::error;
use std::{io, sync::mpsc};

pub mod dispatcher;

fn main() {
    let logger_op = flexi_logger::Logger::try_with_str("info").ok();
    if let Some(log_handle) = logger_op {
        _ = log_handle
            .log_to_file(flexi_logger::FileSpec::default())
            .rotate(
                Criterion::Size(1_000_000),
                Naming::Timestamps,
                Cleanup::KeepLogFiles(1),
            )
            .start()
            .ok();

        let (tx, rx) = mpsc::channel();
        let dispatcher_handle = dispatcher::start(rx);

        if let Err(e) = io::stdin().read_line(&mut String::new()) {
            error!("{e:?}");
        }
        if let Err(e) = tx.send(()) {
            error!("Errors occurred while sending shutdown signal. {e:?}");
        }
        if let Err(e) = dispatcher_handle.join() {
            error!("Errors occurred while joining main thread. {e:?}");
        }
    }
    // Unimplemented service code
    /*
    let service_path = env::current_exe();

    let Ok(service_path) = service_path else {
        error!("Failed to get service executable path. {service_path:?}");
        std::process::exit(1);
    };

    let service_path = service_path.parent().unwrap(); // "This should never give an error" -words said before disaster
    if let Err(e) = env::set_current_dir(service_path) {
        error!("Cannot set working directory. {e:?}")
    }

    if let Err(e) = daemon::run() {
        error!("Daemon exited with error: {e:?}")
    }
    */
}
