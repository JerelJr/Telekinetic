/**
 * Opens a tcp stream to asynchronously read
 * gestures and finger positions and translate
 * them into mouse input
 */
use async_std::io;
use async_std::net::TcpListener;
use enigo::*;
use flexi_logger::{Cleanup, Criterion, Naming};
use futures::{try_join, AsyncReadExt};
use log::{debug, error, info, warn};
use std::error::Error;
use winit::event_loop::EventLoop;

// Constants for input events
const LEFT_PRESS: u8 = 2;
const LEFT_RELEASE: u8 = 3;
const RIGHT_PRESS: u8 = 4;
const RIGHT_RELEASE: u8 = 5;

// TODO: Should send left and right mouse button release events before exit

// Stream for finger position to be translated into mouse position
async fn position_stream() -> Result<(), Box<dyn Error>> {
    let mut buf: [u8; 8] = [0; 8];
    let mut sim = Enigo::new();
    // Get monitor dimensions
    let dimensions = {
        let event_loop = EventLoop::new();
        let monitor = event_loop
            .primary_monitor()
            .ok_or("Primary monitor unavailable")?;
        [monitor.size().width, monitor.size().height]
    };
    let listener = TcpListener::bind("127.0.0.1:1277").await?;

    // Stream loop
    match listener.accept().await {
        Ok((mut stream, _)) => {
            info!("Position client connected");
            let mut prev_frame = [0, 0];
            loop {
                let read_size = AsyncReadExt::read(&mut stream, &mut buf).await?;
                if read_size == 0 {
                    std::process::exit(0);
                }
                let coords: [u32; 2] = [
                    (f32::from_ne_bytes([buf[0], buf[1], buf[2], buf[3]]) * dimensions[0] as f32)
                        as u32,
                    (f32::from_ne_bytes([buf[4], buf[5], buf[6], buf[7]]) * dimensions[1] as f32)
                        as u32,
                ];
                if prev_frame != coords {
                    debug!("{:?}", coords);
                    sim.mouse_move_to(coords[0] as i32, coords[1] as i32);
                }
                prev_frame = coords;
            }
        }
        Err(e) => {
            error!("Error accepting connection ");
            Err(Box::new(e))
        }
    }
}

// Stream for changes in finger state to be translated into mouse events
async fn gesture_stream() -> Result<(), Box<dyn Error>> {
    let mut gesture: [u8; 1] = [0];
    let mut sim = Enigo::new();
    let listener = TcpListener::bind("127.0.0.1:1278").await?;

    // Stream loop
    match listener.accept().await {
        Ok((mut stream, _)) => {
            info!("Gesture client connected");
            loop {
                AsyncReadExt::read(&mut stream, &mut gesture).await?;
                debug!("Gesture data received: {}", gesture[0].clone());
                match gesture {
                    [LEFT_PRESS] => sim.mouse_down(MouseButton::Left),
                    [LEFT_RELEASE] => sim.mouse_up(MouseButton::Left),
                    [RIGHT_PRESS] => sim.mouse_down(MouseButton::Right),
                    [RIGHT_RELEASE] => sim.mouse_up(MouseButton::Right),
                    _ => warn!("Unrecognized data received: {gesture:?}"),
                }
            }
        }
        Err(e) => {
            error!("Error accepting connection ");
            Err(Box::new(e))
        }
    }
}

#[async_std::main]
async fn main() -> io::Result<()> {
    let _log_handle = flexi_logger::Logger::try_with_str("info")
        .expect("Error starting log")
        .log_to_file(flexi_logger::FileSpec::default())
        .rotate(
            Criterion::Size(1_000_000),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(1),
        )
        .start()
        .expect("Error starting log");
    if let Err(e) = try_join!(position_stream(), gesture_stream()) {
        error!("Error when joining main thread. {e:?}");
    }

    Ok(())
}
