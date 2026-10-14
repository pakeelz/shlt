use crate::{app::App, storage::need_update};

mod api;
mod app;
mod error;
mod storage;

#[tokio::main]
async fn main() {
    let mut app = App::new();

    match need_update() {
        Ok(need) => {
            if !need {
                if let Ok(shalat) = storage::read_config() {
                    app.response_state(shalat);
                }
            } else {
                app.set_updating_config_state();
            }
        }
        Err(_e) => {}
    }

    let mut terminal = ratatui::init();

    if let Err(e) = app.run(&mut terminal).await {
        eprintln!("{e}");
    }

    ratatui::restore();
}
