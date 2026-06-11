use std::time::Duration;

use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;

const MAGIC_PURPLE: &str = "#8A2BE2";

pub fn info(message: impl AsRef<str>) {
    eprintln!("{} {}", style("now").color256(93).bold(), message.as_ref());
}

pub fn success(message: impl AsRef<str>) {
    eprintln!("{} {}", "✓".green(), message.as_ref());
}

pub fn warn(message: impl AsRef<str>) {
    eprintln!("{} {}", "warning".yellow(), message.as_ref());
}

pub fn spinner(message: impl Into<String>) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.magenta} {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_spinner())
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.set_message(message.into());
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner
}

#[allow(dead_code)]
pub fn magic_color() -> &'static str {
    MAGIC_PURPLE
}
