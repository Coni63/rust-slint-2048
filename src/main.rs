mod board;

use std::{cell::RefCell, rc::Rc};

use board::Board;
use slint::{ModelRc, SharedString, Weak};

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
    let ui_handle = ui.as_weak();

    update_app(ui_handle.unwrap(), Rc::clone(&game));

    let game_on_restart = Rc::clone(&game);
    ui.on_restart(move |seed| {
        println!("Restarting with seed: {}", seed);

        game_on_restart.borrow_mut().restart(seed.parse().unwrap());

        println!("{}", game_on_restart.borrow());
        // let window = ui_handle.unwrap();

        // update_app(window, Rc::clone(&game_on_restart));
    });

    let game_on_play = Rc::clone(&game);
    ui.on_play(move |direction| {
        println!("Playing with direction: {:?}", direction);

        game_on_play.borrow_mut().apply_action(direction as u8);

        println!("{}", game_on_play.borrow());

        // let window = ui_handle.unwrap();

        // update_app(window, Rc::clone(&game_on_play));
    });

    ui.run()?;
    Ok(())
}
