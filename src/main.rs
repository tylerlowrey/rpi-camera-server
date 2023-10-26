use apriltag::{Detector, Family};
use v4l::{Capabilities, Device};
use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::prelude::MmapStream;
use v4l::video::Capture;

fn main() {
    println!("[rpi-camera-server] Camera server starting...");
    let mut capture_device = Device::new(0)
        .expect("Failed to create a capture device. Make sure the camera is plugged in");

    let Ok(Capabilities { driver, card, bus, version, .. }) = capture_device.query_caps() else {
        panic!("Unable to get capture device information.");
    };

    let Ok(picture_format) = capture_device.format() else {
        panic!("Unable to get capture device picture format.");
    };

    println!("Camera info:\nDriver: {}\nCard: {}\nBus: {}\nVersion: {:?}\nPicture Format {}\n",
             driver, card, bus, version, picture_format);

    let mut stream = MmapStream::new(&capture_device, Type::VideoCapture)
        .expect("Unable to create stream from capture device");

    let Ok(frame) = stream.next() else {
        println!("Bad result from retrieving next frame")
    };

    println!("Frame info [Timestamp: {}, Bytes: {}]", frame.1.timestamp, frame.1.bytesused);

    let apriltag_detector = Detector::builder()
        .add_family_bits(Family::Tag16h5, 1)
        .build()
        .expect("Unable to create an apriltag detector");
}
