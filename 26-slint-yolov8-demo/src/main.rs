use std::{path::PathBuf, sync::mpsc::channel};

use native_dialog::FileDialog;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};

slint::include_modules!();

mod yolov8engine;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_select_orig_pic(move || {
        let ui = ui_handle.unwrap();

        let path = FileDialog::new()
            .set_location("~")
            .add_filter("Pics", &["png", "jpg", "jpeg"])
            .show_open_single_file()
            .unwrap();

        // if selected pic file
        if let Some(path) = path {
            ui.set_orig_image_path(path.to_string_lossy().to_string().into());
            ui.set_orig_image(load_image(path));
        }
    });

    let (sender, receiver) = channel::<(String, String)>();
    let sender1 = sender.clone();
    let sender2 = sender.clone();

    let ui_handle = ui.as_weak();
    let _thread = std::thread::spawn(move || {
        loop {
            let ui_handle = ui_handle.clone();
            let (task, img_path) = receiver.recv().unwrap();
            if task.as_str() == "_exit_" {
                // end of this thread
                return;
            }

            let (task, model) = if task.as_str() == "detect" {
                (
                    yolov8engine::YoloTask::Detect,
                    Some("yolov8m.safetensors".to_string()),
                )
            } else {
                (
                    yolov8engine::YoloTask::Pose,
                    Some("yolov8m-pose.safetensors".to_string()),
                )
            };

            if let Ok(path) = yolov8engine::start_engine(task, model, img_path) {
                _ = slint::invoke_from_event_loop(move || {
                    let ui = ui_handle.unwrap();
                    ui.set_generated_image(load_image(PathBuf::from(path)));
                });
                // ui.set_generated_image(load_image(PathBuf::from(path)));
            } else {
            }
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_probe_objects(move || {
        let ui = ui_handle.unwrap();
        let img_path = ui.get_orig_image_path().to_string();
        println!("{}", img_path);
        _ = sender.send(("detect".to_string(), img_path));
    });

    let ui_handle = ui.as_weak();
    ui.on_probe_poses(move || {
        let ui = ui_handle.unwrap();
        let img_path = ui.get_orig_image_path().to_string();
        println!("{}", img_path);
        _ = sender1.send(("pose".to_string(), img_path));
    });

    ui.window().on_close_requested(move || {
        sender2
            .send(("_exit_".to_string(), "".to_string()))
            .unwrap();
        slint::CloseRequestResponse::HideWindow
    });

    ui.run()
}

fn load_image(path: std::path::PathBuf) -> slint::Image {
    let mut a_image = image::open(path).expect("Error loading image").into_rgba8();

    image::imageops::colorops::brighten_in_place(&mut a_image, 20);

    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        a_image.as_raw(),
        a_image.width(),
        a_image.height(),
    );
    let image = Image::from_rgba8(buffer);

    image
}
