use iced::widget::Container;
use iced_aw::{spinner, Spinner};
use iced_native::{
    column,
    widget::{button, container, text},
    Length, Padding,
};

type Page<'a> = Container<'a, AppEvents>;

#[derive(Debug, Clone)]
pub enum AppEvents {
    Navigate(AppPages),
}

#[derive(Debug, Clone, Default)]
pub enum AppPages {
    #[default]
    Greeting,
    Prerequisites,
}

impl AppPages {
    pub fn navigate(&self) -> iced::Element<'_, AppEvents> {
        return match self {
            AppPages::Greeting => Routes::begin_install(),
            AppPages::Prerequisites => Routes::prerequisites(),
        }
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
    }
}

struct Routes;
impl<'a> Routes {
    pub fn begin_install() -> Page<'a> {
        let content_col = column![
            text("TectoneOS Installer").size(42),
            button(text("Begin installation").size(20))
                .padding(Padding {
                    bottom: 16.0,
                    top: 16.0,
                    left: 30.0,
                    right: 30.0,
                })
                .on_press(AppEvents::Navigate(AppPages::Prerequisites)),
            text("TectoneOS is highly experimental. Here be dragons! Tectone23 shall not be held liable for any damages that might incur."),
        ];

        return container(
            content_col
                .align_items(iced_native::Alignment::Center)
                .spacing(24),
        )
        .center_x()
        .center_y();
    }

    pub fn prerequisites() -> Page<'a> {

        let spinner = Spinner::new();

        let content_col = column![text("Running checks").size(42), spinner];

        return container(content_col.spacing(24));
    }
}
