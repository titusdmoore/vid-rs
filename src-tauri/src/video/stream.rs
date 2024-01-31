use eye::colorconvert::Device;
use eye::hal::format::PixelFormat;
use eye::hal::traits::{Context as _, Device as _, Stream as _};
use eye::hal::{Error, ErrorKind, PlatformContext, Result};
use std::sync::mpsc;
// use std::thread;

pub fn build_stream() -> Result<()> {
    // Create a context
    let ctx = if let Some(ctx) = PlatformContext::all().next() {
        ctx
    } else {
        return Err(Error::new(
            ErrorKind::Other,
            "No platform context available",
        ));
    };

    // Create a list of valid capture devices in the system.
    let dev_descrs = ctx.devices().unwrap();

    // Print the supported formats for each device.
    let dev = ctx.open_device(&dev_descrs[0].uri).unwrap();
    let dev = Device::new(dev).unwrap();
    let stream_descr = dev
        .streams()
        .unwrap()
        .into_iter()
        .reduce(|s1, s2| {
            // Choose RGB with 8 bit depth
            if s1.pixfmt == PixelFormat::Rgb(24) && s2.pixfmt != PixelFormat::Rgb(24) {
                return s1;
            }

            // Strive for HD (1280 x 720)
            let distance = |width: u32, height: u32| {
                f32::sqrt(((1280 - width as i32).pow(2) + (720 - height as i32).pow(2)) as f32)
            };

            if distance(s1.width, s1.height) < distance(s2.width, s2.height) {
                s1
            } else {
                s2
            }
        })
        .unwrap();

    if stream_descr.pixfmt != PixelFormat::Rgb(24) {
        return Err(Error::new(ErrorKind::Other, "No RGB3 streams available"));
    }

    println!("Selected stream:\n{:?}", stream_descr);

    // Start the stream
    let mut stream = dev.start_stream(&stream_descr).unwrap();

    let (tx, rx) = mpsc::channel::<Vec<u8>>();

    // thread::spawn(move || loop {
    //     let buf = stream.next().unwrap().unwrap();
    //     tx.send(buf.to_vec()).unwrap();
    // });
    //
    // thread::spawn(move || loop {
    //     let buf = rx.recv().unwrap();
    //     set_canvas_content.update(|x| *x = buf);
    // });

    Ok(())
}
