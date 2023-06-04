mod app;
mod error;

use app::App;
use eframe::epaint::vec2;

pub(crate) type Result<T> = std::result::Result<T, error::Error>;

fn main() -> Result<()> {
    eframe::run_native(
        "Encoder",
        eframe::NativeOptions {
            initial_window_size: Some(vec2(700., 700.)),
            min_window_size: Some(vec2(700., 700.)),
            ..Default::default()
        },
        Box::new(|cc| Box::new(App::new(cc))),
    )?;

    Ok(())
}
