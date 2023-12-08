#![allow(unused)]

use std::sync::mpsc::channel;

mod token_output_stream;
mod llmengin;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    let (sender, receiver) = channel::<String>();
    let sender1 = sender.clone();

    let _thread = std::thread::spawn(move || {
        
        if let Err(_) = llmengin::start_engine(ui_handle, receiver) {
            // process before exit.
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_send_ask_content(move |content| {
        update_dialog(ui_handle.clone(), content.to_string());
        sender.send(content.to_string()).unwrap();
    });

    ui.window().on_close_requested(move || {
        sender1.send("_exit_".to_string()).unwrap();
        slint::CloseRequestResponse::HideWindow
    });

    ui.run()
}

fn update_dialog(ui_handle: slint::Weak<AppWindow>, msg: String) {
    _ = slint::invoke_from_event_loop(move || {
        let ui_handle = ui_handle.unwrap();
        let old_content = ui_handle.get_dialog();
        ui_handle.set_dialog(old_content + &msg + "\n");
    });
}

fn update_dialog_without_ln(ui_handle: slint::Weak<AppWindow>, msg: String) {
    _ = slint::invoke_from_event_loop(move || {
        let ui_handle = ui_handle.unwrap();
        let old_content = ui_handle.get_dialog();
        ui_handle.set_dialog(old_content + &msg);
    });
}