mod board;

use board::Board;
use slint::{ModelRc, SharedString, Weak};

slint::include_modules!();

fn update_app(ui: AppWindow, game: &Board) {
    let grid: ModelRc<i32> = ModelRc::from(game.board);
    let sstr = SharedString::from(game.seed.to_string());

    ui.set_values(grid);
    ui.set_score(game.score as i32);
    ui.set_seed(sstr);
}

fn main() -> Result<(), slint::PlatformError> {
    let mut game = Board::new(32);

    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    update_app(ui_handle.unwrap(), &game);

    ui.on_restart(move |seed| {
        println!("Restarting with seed: {}", seed);

        // game.restart(seed.parse().unwrap());

        // let window = ui_handle.unwrap();

        // update_app(window, &game);
    });

    ui.on_play(move |direction| {
        println!("Playing with direction: {:?}", direction);

        game.apply_action(direction as u8);

        let window = ui_handle.unwrap();

        update_app(window, &game);
    });

    ui.run()
}
