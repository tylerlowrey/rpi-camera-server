use apriltag::{Detector};
use v4l::{Capabilities, Device};
use v4l::buffer::Type;
use v4l::io::traits::{CaptureStream, Stream};
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

    println!("Camera info:\nDriver: {}\nCard: {}\nBus: {}\nVersion: {:?}\nPicture Format:\n{}\n",
             driver, card, bus, version, picture_format);

    let mut stream = Stream::with(&mut capture_device, Type::VideoCapture)
        .expect("Unable to create stream from capture device");

    stream.start().unwrap();
    println!("Created stream from capture device");

    stream.next().unwrap();

    println!("First frame consumed");

    let Ok((buffer, metadata)) = stream.next() else {
        println!("Bad result from retrieving next frame");
        return;
    };

    println!("Second frame consumed");

    println!("Frame info [Timestamp: {}, Bytes: {}]", metadata.timestamp, metadata.bytesused);

    let img_buffer = image::ImageBuffer::<image::Rgb<u8>, _>::from_raw(
        picture_format.width,
        picture_format.height,
        buffer.to_vec()
    ).unwrap();

    let dynamic_img = image::DynamicImage::ImageRgb8(img_buffer);
    dynamic_img.save("~/captured_image.png").unwrap();
}
