use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub async fn with_spinner<F, T, E>(message: &str, future: F) -> Result<T, E>
where
    F: Future<Output = Result<T, E>>,
{
    let spinner = ProgressBar::new_spinner().with_message(message.to_string());
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );

    spinner.enable_steady_tick(Duration::from_millis(100));
    let result = future.await;

    spinner.finish_and_clear();
    result
}
