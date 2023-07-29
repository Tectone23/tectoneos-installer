use iced::widget::{scrollable, Container};
use iced_aw::Spinner;
use iced_native::{
    column, row,
    widget::{button, container, horizontal_space, pick_list, text},
    Color, Length, Padding, Theme,
};

use crate::{
    events,
    installer::phone::{Connected, Phone},
    App,
};

type Page<'a> = Container<'a, AppEvents>;

#[derive(Debug, Clone)]
pub enum AppEvents {
    Navigate(AppPages),
    TaskEvent(events::Event),
    OpenUrl(&'static str),
    SetPhone(Phone<Connected>),
}

#[derive(Debug, Clone, Default)]
pub enum AppPages {
    #[default]
    Greeting,
    Prerequisites,
    Eula,
    DeviceSelect,
    Missing,
}

impl AppPages {
    pub fn navigate(&self, app: &App) -> iced::Element<'_, AppEvents> {
        return match self {
            AppPages::Greeting => Routes::begin_install(),
            AppPages::Prerequisites => Routes::prerequisites(),
            AppPages::Eula => Routes::eula(),
            AppPages::DeviceSelect => Routes::device_select(&app.current_phone),
            AppPages::Missing => Routes::missing(),
        }
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding::new(16.0))
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

    pub fn device_select(phone: &Option<Phone<Connected>>) -> Page<'a> {
        let devices = Phone::get_devices();

        let list = pick_list(devices, phone.clone(), |phone| AppEvents::SetPhone(phone));

        return container(column![
            text("Install TectoneOS on the selected device"),
            row![
                list,
                button("Refresh devices").on_press(AppEvents::Navigate(AppPages::DeviceSelect))
            ],
        ])
        .into();
    }

    pub fn prerequisites() -> Page<'a> {
        let spinner = Spinner::new()
            .width(Length::Fixed(24.00))
            .height(Length::Fixed(24.00));

        let content_col = column![text("We are just running some checks").size(32), spinner,]
            .align_items(iced_native::Alignment::Center);

        return container(content_col.spacing(24)).center_x().center_y();
    }

    pub fn eula() -> Page<'a> {
        let content_col = column![
            container(
                scrollable(text("EULA"))
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(16)
            .style(iced_native::theme::Container::Custom(Box::new(EulaStyle))),
            row![
                horizontal_space(Length::Fill),
                button("Read online").on_press(AppEvents::OpenUrl("https://youtu.be/sFUmPSyG61c")),
                button("Go back").on_press(AppEvents::Navigate(AppPages::Greeting)),
                button("Agree and proceed").on_press(AppEvents::Navigate(AppPages::DeviceSelect)),
            ]
            .spacing(24)
            .width(Length::Fill),
        ]
        .spacing(24)
        .align_items(iced_native::Alignment::Center);

        return container(content_col).into();
    }

    pub fn missing() -> Page<'a> {
        let content_col = column![
            text("System requirements not met").size(32),
            text("Make sure Android SDK Platform Tools are installed"),
            button("More info").on_press(AppEvents::OpenUrl("https://developer.android.com/tools/releases/platform-tools")),
        ];

        return container(content_col.spacing(24));
    }
}

struct EulaStyle;

impl container::StyleSheet for EulaStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        return container::Appearance {
            text_color: None,
            background: Some(iced_native::Background::Color(Color::from_rgb8(47, 50, 55))),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        };
    }
}
