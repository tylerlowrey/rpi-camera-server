use v4l::{Capabilities, Device};

fn main() {
    println!("[rpi-camera-server] Camera server starting...");
    let mut capture_device = Device::new(0)
        .expect("Failed to create a capture device. Make sure the camera is plugged in");

    let Ok(Capabilities { driver, card, bus, version, .. }) = capture_device.query_caps() else {
        panic!("Unable to get capture device information.");
    };

    println!("Camera info:\nDriver: {}\nCard: {}\nBus: {}\nVersion: {:?}\n",
             driver, card, bus, version);
}
