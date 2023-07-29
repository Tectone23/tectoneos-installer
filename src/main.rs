mod events;
mod routes;
mod installer;
mod utils;

use std::error::Error;

use events::{task_worker, Connection};
use iced::{executor, Application, Settings};
use iced_native::{Command, Theme};
use installer::phone::{Phone, Connected};
use routes::{AppEvents, AppPages};

#[derive(Default)]
pub struct App {
    page: AppPages,
    event_sender: Option<Connection>,
    pub current_phone : Option<Phone<Connected>>,
}

impl Application for App {
    type Message = AppEvents;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        return (Self::default(), Command::none());
    }

    fn title(&self) -> String {
        return "TectoneOS Installer".to_string();
    }

    fn theme(&self) -> iced_native::Theme {
        return iced_native::Theme::Dark;
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppEvents::Navigate(page) => self.page = page,
            AppEvents::TaskEvent(event) => match event {
                events::Event::Connected(conn) => self.event_sender = Some(conn),
                events::Event::Navigate(page) => self.page = page,
            },
            AppEvents::OpenUrl(url) => {
                let _ = open::that(url);
            }
            AppEvents::SetPhone(phone) => self.current_phone = Some(phone),
        };
        return Command::none();
    }

    fn subscription(&self) -> iced_native::Subscription<Self::Message> {
        return task_worker().map(AppEvents::TaskEvent);
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        return match self.page {
            AppPages::Prerequisites => {
                if let Some(sender) = &self.event_sender {
                    let mut sender = sender.clone();
                    sender.send(events::Message::CheckPrereq);
                }
                self.page.navigate(&self)
            }
            _ => self.page.navigate(&self),
        };
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    App::run(Settings::default())?;

    return Ok(());
}
