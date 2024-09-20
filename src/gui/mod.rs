use std::collections::HashMap;

use self::{
    analysis_database_body::{AnalysisDatabaseBody, AnalysisDatabaseMessage},
    config_body::{ConfigBody, ConfigMessage},
    decrypt_body::{DecryptBody, DecryptMessage},
    get_database_body::{GetDatabaseBody, GetDatabaseMessage},
    show_user_info_body::{ShowUserInfoBody, ShowUserInfoMessage},
};
use iced::{
    widget::{Button, Column, Container, Image, Row, Space},
    Length, Subscription,
};

mod analysis_database_body;
mod config_body;
mod decrypt_body;
mod get_database_body;
mod gui_util;
mod show_user_info_body;

pub struct WxDumpGui {
    id: iced::window::Id,
    body: Body,
    config_body: ConfigBody,
    theme: iced::theme::Theme,
    show_user_info_body: ShowUserInfoBody,
    get_database_body: GetDatabaseBody,
    decrypt_body: DecryptBody,
    analysis_database_body: AnalysisDatabaseBody,
    image_id: HashMap<iced::window::Id, Vec<u8>>,
}

impl WxDumpGui {
    // type Executor = iced::executor::Default;

    // type Message = Message;

    // type Theme = iced::theme::Theme;

    // type Flags = super::Flags;

    pub fn new() -> (Self, iced::Task<Message>) {
        let icon: &[u8] = include_bytes!("../image/icon.png");
        let (id, task) = iced::window::open(iced::window::Settings {
            icon: iced::window::icon::from_file_data(icon, None).ok(),
            ..iced::window::Settings::default()
        });
        (
            WxDumpGui {
                id,
                body: Body::Config,
                config_body: ConfigBody::new(),
                theme: iced::theme::Theme::default(),
                show_user_info_body: ShowUserInfoBody::new(),
                get_database_body: GetDatabaseBody::new(),
                decrypt_body: DecryptBody::new(),
                analysis_database_body: AnalysisDatabaseBody::new(),
                image_id: HashMap::new(),
            },
            iced::Task::<Message>::batch(vec![
                iced::font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(|_| Message::FontLoaded),
                iced::font::load(iced_aw::NERD_FONT_BYTES).map(|_| Message::FontLoaded),
                task.map(Message::OptionWindow),
            ]),
        )
    }

    pub fn title(&self, id: iced::window::Id) -> String {
        if self.image_id.keys().find(|k| k == &&id).is_some() {
            return "查看图片".to_owned();
        }
        "微信记录解密查看器".to_owned()
    }

    pub fn theme(&self, _id: iced::window::Id) -> iced::theme::Theme {
        self.theme.clone()
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::ConfigMessage(msg) => self.config_body.update(msg),
            Message::ButtonConfig => self.body = Body::Config,
            Message::ButtonShowUserInfo => {
                let r = self
                    .show_user_info_body
                    .update(ShowUserInfoMessage::UpdateWeChatInfo, &self.config_body);
                self.body = Body::ShowUserInfo;
                return r;
            }
            Message::ButtonGetDatabase => {
                let r = self.get_database_body.update(
                    GetDatabaseMessage::UpdateGetDatabase,
                    &self.config_body,
                    &self.show_user_info_body,
                );
                self.body = Body::GetDatabase;
                return r;
            }
            Message::GetDatabaseMessage(msg) => {
                return self.get_database_body.update(
                    msg,
                    &self.config_body,
                    &self.show_user_info_body,
                )
            }
            Message::ShowUserInfoMessage(msg) => {
                return self.show_user_info_body.update(msg, &self.config_body);
            }
            Message::CheckArcData => match self.body {
                Body::Config => todo!(),
                Body::ShowUserInfo => return self.show_user_info_body.check_scroll(),
                Body::GetDatabase => return self.get_database_body.check_scroll(),
                Body::Decrypt => return self.decrypt_body.check_scroll(),
                Body::AnalysisDatabase => todo!(),
            },
            Message::DecryptMessage(msg) => {
                return self
                    .decrypt_body
                    .update(msg, &self.config_body, &self.show_user_info_body);
            }
            Message::ButtonDecrypt => {
                let r = self.decrypt_body.update(
                    DecryptMessage::UpdateDecrypt,
                    &self.config_body,
                    &self.show_user_info_body,
                );
                self.body = Body::Decrypt;
                return r;
            }
            Message::ButtonAnalysisDatabase => {
                let r = self.analysis_database_body.update(
                    AnalysisDatabaseMessage::UpdateAnalysisDatabase,
                    &self.config_body,
                    &self.theme,
                );
                self.body = Body::AnalysisDatabase;
                return r;
            }
            Message::AnalysisDatabaseMessage(msg) => {
                return self
                    .analysis_database_body
                    .update(msg, &self.config_body, &self.theme)
            }
            Message::OpenImage(image) => {
                let (id, command) = iced::window::open(iced::window::settings::Settings::default());
                self.image_id.insert(id, image);
                return command.map(Message::OptionWindow);
            }
            Message::OptionWindow(_) => {}
            Message::CloseWindow(id) => {
                if id == self.id {
                    return iced::exit();
                }
                self.image_id.remove(&id);
            }
            Message::FontLoaded => {}
            Message::ButtonChangeTheme => {
                self.theme = match self.theme {
                    iced::theme::Theme::Light => iced::theme::Theme::Dark,
                    iced::theme::Theme::Dark => iced::theme::Theme::Light,
                    _ => todo!(),
                }
            }
        }
        iced::Task::<Message>::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // let event = iced::window::close_events(|event, _| match event {
        //     iced::Event::Window(id, event) => match event {
        //         iced::window::Event::Closed => Some(Message::CloseWindow(id)),
        //         _ => None,
        //     },
        //     _ => None,
        // });
        iced::Subscription::batch(vec![
            iced::window::close_events().map(Message::CloseWindow),
            match self.body {
                Body::Config => iced::Subscription::none(),
                Body::ShowUserInfo => {
                    if self.show_user_info_body.check_command_running() {
                        iced::time::every(std::time::Duration::from_millis(10))
                            .map(|_| Message::CheckArcData)
                    } else {
                        iced::Subscription::none()
                    }
                }
                Body::GetDatabase => {
                    if self.get_database_body.check_command_running() {
                        iced::time::every(std::time::Duration::from_millis(10))
                            .map(|_| Message::CheckArcData)
                    } else {
                        iced::Subscription::none()
                    }
                }
                Body::Decrypt => {
                    if self.decrypt_body.check_command_running() {
                        iced::time::every(std::time::Duration::from_millis(10))
                            .map(|_| Message::CheckArcData)
                    } else {
                        iced::Subscription::none()
                    }
                }
                Body::AnalysisDatabase => iced::Subscription::none(),
            },
        ])
    }
    pub fn view(
        &self,
        id: iced::window::Id,
    ) -> iced::Element<'_, Message, iced::theme::Theme, iced::Renderer> {
        if let Some(image) = self.image_id.get(&id) {
            return Container::new(
                iced::widget::scrollable::Scrollable::new(Image::new(
                    iced::widget::image::Handle::from_bytes(image.clone()),
                ))
                .direction(iced::widget::scrollable::Direction::Both {
                    vertical: iced::widget::scrollable::Scrollbar::new(),
                    horizontal: iced::widget::scrollable::Scrollbar::new(),
                })
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .into();
        }
        let col = Column::new()
            .push(Space::with_height(10))
            .push(
                match self.body {
                    Body::Config => self.config_body.draw(),
                    Body::ShowUserInfo => self.show_user_info_body.draw(),
                    Body::GetDatabase => self.get_database_body.draw(),
                    Body::Decrypt => self.decrypt_body.draw(),
                    Body::AnalysisDatabase => self.analysis_database_body.draw(),
                }
                .height(Length::Fill),
            )
            .push(
                Row::new()
                    .spacing(5.0)
                    .height(Length::Fixed(40.0))
                    .push(Space::with_width(Length::Fixed(5.0)))
                    .push(Button::new("配置").on_press(Message::ButtonConfig))
                    .push(Button::new("查看用户信息").on_press_maybe(
                        if (self.config_body.process_id.is_some()
                            || self.config_body.process_name.trim().len() != 0)
                            && self.config_body.module_name.trim().len() != 0
                        {
                            Some(Message::ButtonShowUserInfo)
                        } else {
                            None
                        },
                    ))
                    .push(Button::new("备份数据库").on_press_maybe(
                        if self.config_body.save_path.trim().len() != 0
                            && !self.decrypt_body.check_command_running()
                        {
                            Some(Message::ButtonGetDatabase)
                        } else {
                            None
                        },
                    ))
                    .push(Button::new("解密").on_press_maybe(
                        if self.config_body.save_path.trim().len() != 0
                            && self.config_body.decrypt_path.trim().len() != 0
                            && !self.get_database_body.check_command_running()
                        {
                            Some(Message::ButtonDecrypt)
                        } else {
                            None
                        },
                    ))
                    .push(Button::new("查看微信消息").on_press_maybe(
                        if !self.decrypt_body.check_command_running()
                            && self.config_body.decrypt_path.trim().len() != 0
                        {
                            Some(Message::ButtonAnalysisDatabase)
                        } else {
                            None
                        },
                    ))
                    .push(Button::new("切换主题").on_press(Message::ButtonChangeTheme)),
            );
        Container::new(col).into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonConfig,
    ConfigMessage(ConfigMessage),
    ShowUserInfoMessage(ShowUserInfoMessage),
    ButtonShowUserInfo,
    ButtonGetDatabase,
    GetDatabaseMessage(GetDatabaseMessage),
    CheckArcData,
    ButtonDecrypt,
    DecryptMessage(DecryptMessage),
    ButtonAnalysisDatabase,
    AnalysisDatabaseMessage(AnalysisDatabaseMessage),
    OpenImage(Vec<u8>),
    OptionWindow(iced::window::Id),
    CloseWindow(iced::window::Id),
    FontLoaded,
    ButtonChangeTheme,
}

enum Body {
    Config,
    ShowUserInfo,
    GetDatabase,
    Decrypt,
    AnalysisDatabase,
}
