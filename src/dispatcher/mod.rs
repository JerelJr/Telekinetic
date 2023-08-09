use log::{error, info, warn};
use std::{
    env,
    path::PathBuf,
    process::{exit, Command},
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn start(shutdown_rx: Receiver<()>) -> JoinHandle<()> {
    thread::spawn(|| run(shutdown_rx))
}

fn run(shutdown_rx: Receiver<()>) {
    let dir: Option<PathBuf> = match env::current_exe() {
        Ok(path) => path.parent().map(|p| p.to_path_buf()),
        Err(e) => {
            error!("Error determining executable path. {e:?}");
            exit(1);
        }
    };
    let Some(dir) = dir else {
            error!("Unable to determine parent directory.");
            exit(1);
    };

    info!("Working directory: {:?}.", dir.as_path());

    if let Err(e) = env::set_current_dir(dir) {
        error!("Error changing working directory. {e:?}");
        exit(1);
    }
    let mut mouse_sim = match Command::new("./mouse_simulator").spawn() {
        Ok(child) => child,
        Err(e) => {
            error!("Error spawning mouse simulator. {e:?}");
            exit(1);
        }
    };
    info!("Mouse simulator spawned.");
    thread::sleep(Duration::from_millis(500)); // Give the server time to start

    let mut hand_tracker = match Command::new("./hand-tracker").spawn() {
        Ok(child) => child,
        Err(e) => {
            error!("Error spawning hand tracker. {e:?}");

            if let Err(e) = mouse_sim.kill() {
                warn!("mouse-simulator already terminated: {e:?}");
            }

            exit(1);
        }
    };
    info!("Hand tracker spawned.");

    if let Err(e) = shutdown_rx.recv() {
        error!("{e:?}");
    }

    if let Err(e) = hand_tracker.kill() {
        warn!("hand-tracker cannot be terminated: {e:?}");
    }
    if let Err(e) = mouse_sim.kill() {
        warn!("mouse-simulator cannot be terminated: {e:?}");
    }
}
