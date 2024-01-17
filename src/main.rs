mod board;

use std::{cell::RefCell, rc::Rc};

use board::Board;
use slint::{ModelRc, SharedString};

slint::include_modules!();

fn update_app(ui: AppWindow, game: Rc<RefCell<Board>>) {
    let grid: ModelRc<i32> = ModelRc::from(game.borrow().board);
    let sstr = SharedString::from(game.borrow().seed.to_string());

    ui.set_values(grid);
    ui.set_score(game.borrow().score as i32);
    ui.set_seed(sstr);
}

fn main() -> Result<(), slint::PlatformError> {
    let game = Rc::new(RefCell::new(Board::new(32)));
    let ui = AppWindow::new()?;

    let ui_handle1 = ui.as_weak();
    let ui_handle2 = ui.as_weak();
    let ui_handle3 = ui.as_weak();

    let game_on_restart = Rc::clone(&game);
    let game_on_play = Rc::clone(&game);

    ui.on_restart(move |seed| {
        ui_handle2.unwrap().set_game_over(false);
        game_on_restart.borrow_mut().restart(seed.parse().unwrap());
        update_app(ui_handle2.unwrap(), Rc::clone(&game_on_restart));
    });

    ui.on_play(move |direction| {
        game_on_play.borrow_mut().apply_action(direction as u8);

        update_app(ui_handle3.unwrap(), Rc::clone(&game_on_play));

        if game_on_play.borrow().is_game_over() {
            ui_handle3.unwrap().set_game_over(true);
        }
    });

    ui.on_exit(|| {
        std::process::exit(0);
    });

    // setup and run the UI
    update_app(ui_handle1.unwrap(), Rc::clone(&game));
    ui.run()?;
    Ok(())
}
