mod app;

use app::NixTerminalApp;
use iced::Application;

fn main() -> iced::Result {
    let settings = iced::Settings {
        antialiasing: true,
        window: iced::window::Settings {
            size: (960, 600),
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    };

    NixTerminalApp::run(settings)
}
