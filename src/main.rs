mod routes;
mod spinner;

use std::error::Error;

use iced::{Sandbox, Settings};
use routes::{AppEvents, AppPages};

#[derive(Default)]
struct App {
    page: AppPages,
}


impl Sandbox for App {
    type Message = AppEvents;

    fn new() -> Self {
        return Self::default();
    }

    fn title(&self) -> String {
        return "TectoneOS Installer".to_string();
    }

    fn theme(&self) -> iced_native::Theme {
        return iced_native::Theme::Dark;
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppEvents::Navigate(page) => self.page = page,
        };
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        return self.page.navigate();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    App::run(Settings::default())?;

    return Ok(());
}
