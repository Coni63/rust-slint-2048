slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    // ui.on_request_increase_value(move || {
    //     let ui = ui_handle.unwrap();
    //     ui.set_counter(ui.get_counter() + 1);
    // });
    ui.on_restart(|seed| {
        println!("Restarting with seed: {}", seed);

        // change grid and also seed + (check proper update of seed)
    });

    ui.on_play(|direction| {
        println!("Playing with direction: {:?}", direction);

        // change grid and also seed + (check proper update of seed)
    });

    ui.run()
}
