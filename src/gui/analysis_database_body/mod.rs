use std::{path::Path, fs::read_dir, collections::HashMap};

use cpal::Stream;
use diesel::SqliteConnection;
use iced::{widget::{Container, Space, Column, Text, Button, Scrollable, Row, Image, TextInput}, Length, Color};
use iced_runtime::core::{text::Shaping, image::Handle};

use self::module::module_macro_msg::{Session, Contact};

use super::{Message, config_body::ConfigBody, gui_util::{set_col_with_text, self}};

mod schema;
mod module;

pub struct AnalysisDatabaseBody {
    decrypted_path: String,
    analysis_database_err_msg: Option<String>,
    conn: Option<Conn>,
    session_list: Vec<Session>,
    body: Body,
    contact: HashMap<Option<String>, Contact>,
    analysis_running: bool,
    msg_getting: bool,
    msg_list: Vec<(usize,module::module_msg::Msg)>,
    message_title: String,
    message_scroll_id: iced::widget::scrollable::Id,
    session_scroll_id: iced::widget::scrollable::Id,
    wechat_path: Option<String>,
    msg_page: usize,
    msg_page_input: String,
    audio_err_texts: HashMap<i64,String>,
    audio_stream: Option<(Stream,i64)>,
}

impl AnalysisDatabaseBody {
    pub fn new() -> Self {
        Self {
            decrypted_path: String::new(),
            analysis_database_err_msg: None,
            conn: None,
            session_list: vec![],
            body: Body::Session,
            contact: HashMap::new(),
            analysis_running: false,
            msg_getting: false,
            msg_list: vec![],
            message_title: String::new(),
            message_scroll_id: iced::widget::scrollable::Id::new("analysis_message_scroll"),
            session_scroll_id: iced::widget::scrollable::Id::new("analysis_session_scroll"),
            wechat_path: None,
            msg_page: 0,
            msg_page_input: "1".to_string(),
            audio_err_texts: HashMap::new(),
            audio_stream: None,
        }
    }

    pub fn update(&mut self, msg: AnalysisDatabaseMessage, config_body: &ConfigBody) -> iced::Command<Message> {
        match msg {
            AnalysisDatabaseMessage::UpdateAnalysisDatabase => {
                self.decrypted_path = config_body.decrypt_path.trim().to_owned();
                self.wechat_path = config_body.wechat_dir.to_owned();
                iced::Command::none()
            },
            AnalysisDatabaseMessage::ButtonAnalysis => {
                self.analysis_running = true;
                match self.get_conn() {
                    Ok(mut conn) => {
                        match module::module_macro_msg::get_sessions(&mut conn.macro_msg_conn) {
                            Ok(v) => {
                                self.session_list = v;
                                self.body = Body::Session;
                                match module::module_macro_msg::get_contact(&mut conn.macro_msg_conn) {
                                    Ok(map) => self.contact = map,
                                    Err(e) => self.analysis_database_err_msg = Some(e.to_string()),
                                } 
                            },
                            Err(e) => self.analysis_database_err_msg = Some(e.to_string()),
                        }
                        self.conn = Some(conn);
                    },
                    Err(r) => {
                        self.analysis_database_err_msg = Some(r.to_string());
                    },
                }
                self.analysis_running = false;
                iced::Command::none()
            },
            AnalysisDatabaseMessage::ButtonSession(index, message_title) => {
                self.msg_getting = true;
                match module::module_msg::get_msg_by_user_name(&self.session_list[index].str_usr_name, &mut self.conn.as_mut().unwrap().msg_conn_map) {
                    Ok(s) => {
                        self.msg_list = s;
                        self.msg_page = self.msg_list.len() / 100;
                        self.msg_page_input = (self.msg_page + 1).to_string();
                        self.message_title = message_title;
                        self.body = Body::Msg;
                        self.msg_getting = false;
                        return iced::widget::scrollable::snap_to::<Message>(self.message_scroll_id.clone(), iced::widget::scrollable::RelativeOffset{ x: 0.0, y: 1.0 });
                    },
                    Err(e) => self.analysis_database_err_msg = Some(e.to_string()),
                }
                self.msg_getting = false;
                iced::Command::none()
            },
            AnalysisDatabaseMessage::ButtonBack => {
                self.msg_list = vec![];
                self.msg_page = 0;
                self.msg_page_input = (self.msg_page + 1).to_string();
                self.body = Body::Session;
                iced::Command::none()
            },
            AnalysisDatabaseMessage::ButtonMsgPrev => {
                self.msg_page -= 1;
                self.msg_page_input = (self.msg_page + 1).to_string();
                iced::Command::none()
            },
            AnalysisDatabaseMessage::ButtonMsgNext => {
                self.msg_page += 1;
                self.msg_page_input = (self.msg_page + 1).to_string();
                iced::Command::none()
            },
            AnalysisDatabaseMessage::InputMsgPage(s) => {
                self.msg_page_input = s;
                iced::Command::none()
            },
            AnalysisDatabaseMessage::ButtonMsgJumpTo(p) => {
                self.msg_page = p;
                self.msg_page_input = (self.msg_page + 1).to_string();
                iced::Command::none()
            },
            AnalysisDatabaseMessage::ButtonMsgPlayAudio(id , key) => {
                if let Some(conn) = self.conn.as_mut().unwrap().media_msg_conn_map.get_mut(&key) {
                    match gui_util::play_audio(|sample_rate| module::module_media_msg::get_audio_pcm(id,sample_rate,conn)) {
                        Ok(stream) => {
                            self.audio_stream = Some((stream,id));
                        },
                        Err(e) => {
                            self.audio_err_texts.insert(id, e.to_string());
                        },
                    }
                } else {
                    self.audio_err_texts.insert(id, "未找到音频数据库".to_string());
                }
                iced::Command::none()
            },
            AnalysisDatabaseMessage::ButtonMsgStopAudio => {
                self.audio_stream = None;
                iced::Command::none()
            },
            AnalysisDatabaseMessage::ButtonTestSilkSdk => {
                module::module_media_msg::test_for_silk(self.conn.as_mut().unwrap().media_msg_conn_map.get_mut(&0).unwrap()).unwrap();
                iced::Command::none()
            },
        }
    }

    fn get_conn(&self) -> Result<Conn,anyhow::Error> {
        let decrypted_path = Path::new(self.decrypted_path.trim());
        let mut msg_conn_map = HashMap::new();
        let mut media_msg_conn_map = HashMap::new();
        let mut macro_msg_conn = None;
        for entry in read_dir(decrypted_path)? {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with("decrypted_MSG") && filename.ends_with(".db") {
                        if let Ok(index) = filename.replace("decrypted_MSG", "").replace(".db", "").parse::<usize>() {
                            msg_conn_map.insert(index, module::get_conn(entry.path().display())?);
                        }
                    } else if filename.starts_with("decrypted_MediaMSG") && filename.ends_with(".db") {
                        if let Ok(index) = filename.replace("decrypted_MediaMSG", "").replace(".db", "").parse::<usize>() {
                            media_msg_conn_map.insert(index, module::get_conn(entry.path().display())?);
                        }
                    } else if filename == "decrypted_MicroMsg.db" {
                        macro_msg_conn = Some(module::get_conn(entry.path().display())?);
                    }
                }
            }
        }
        Ok(Conn { msg_conn_map, media_msg_conn_map, macro_msg_conn: macro_msg_conn.ok_or(anyhow::anyhow!("未找到MicroMsg数据库"))? })
    }
    fn draw_message(&self) -> Container<Message> {
        Container::new({
            let mut col = Column::new().spacing(10);
            let list = if (self.msg_page+1) * 100 > self.msg_list.len() {
                &self.msg_list[self.msg_page * 100..]
            } else {
                &self.msg_list[self.msg_page * 100..(self.msg_page+1) * 100]
            };
            for (index,msg) in list {
                col = col.push({
                    let mut row = Row::new();
                    let container = Container::new({
                        let mut col = Column::new().spacing(3);
                        if let Some(byte_ex) = &msg.bytes_extra {
                            if let Some(data) = byte_ex.map.get(&1) {
                                if let Some(data) = data.get(0) {
                                    if let Some(contact) = self.contact.get(&Some(data.to_owned())) {
                                        let mut text = contact.nick_name.as_ref().cloned().unwrap_or("".to_string());
                                        if let Some(remark) = &contact.remark {
                                            if remark.len() > 0 {
                                                text = format!("{}({})",text,remark);
                                            }
                                        }
                                        col = col.push(Text::new(text).size(19).shaping(Shaping::Advanced));
                                    }
                                }
                            }
                        }
                        col = col.push(
                                Space::with_height(3)
                            ).push(
                                match msg.load_msg_data(&self.wechat_path) {
                                    module::module_msg::MsgData::Text(s) => {
                                        Container::new(
                                            Text::new(s).size(18).shaping(Shaping::Advanced)
                                        )
                                    },
                                    module::module_msg::MsgData::Image(image) => {
                                        match image {
                                            Ok(image) => {
                                                Container::new(Image::new(Handle::from_memory(image)))
                                            },
                                            Err(err) => {
                                                Container::new(Text::new(format!("图片获取失败，错误信息： {}",err.to_string())).size(18))
                                            },
                                        }
                                    },
                                    module::module_msg::MsgData::Other(s) => {
                                        Container::new(
                                            Text::new(s).size(18).shaping(Shaping::Advanced)
                                        )
                                    },
                                    module::module_msg::MsgData::Audio(id) => {
                                        Container::new(
                                            {
                                                let mut col = Column::new();
                                                col = col.push(
                                                    {
                                                        let mut row = Row::new().spacing(5);
                                                        row = row.push(
                                                            Button::new("播放音频").on_press(
                                                                Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::ButtonMsgPlayAudio(id, *index))
                                                            )
                                                        );
                                                        if let Some((_,playing_id)) = self.audio_stream {
                                                            if playing_id == id {
                                                                row = row.push(
                                                                    Button::new("停止播放").on_press(
                                                                        Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::ButtonMsgStopAudio)
                                                                    )
                                                                )
                                                            }
                                                        }
                                                        row
                                                    }
                                                );
                                                if let Some(s) = self.audio_err_texts.get(&id) {
                                                    col = col.push(
                                                        Text::new(s).style(iced::theme::Text::Color(Color::from_rgb(1.0, 0.0, 0.0)))
                                                    )
                                                }
                                                col
                                            }
                                        )
                                    },
                                }
                            ).push(
                                Space::with_height(3)
                            );
                        col
                    }).width(700);
                    if msg.is_sender == Some(1) {
                        row = row
                            .push(Space::with_width(Length::Fill))
                            .push(container.align_x(iced::alignment::Horizontal::Right).style(iced::theme::Container::Custom(Box::new(MsgContainerTheme::Right))))
                            .push(Space::with_width(20))
                        ;
                    } else {
                        row = row
                            .push(Space::with_width(10))
                            .push(container.align_x(iced::alignment::Horizontal::Left).style(iced::theme::Container::Custom(Box::new(MsgContainerTheme::Left))))
                            .push(Space::with_width(Length::Fill))
                        ;
                    }
                    row.width(Length::Fill)
                }).width(Length::Fill)
            }
            col
        })
    }
    fn draw_session(&self) -> Container<Message> {
        Container::new({
            let mut col = Column::new();
            for (index,session) in self.session_list.iter().enumerate() {
                let content = self.contact.get(&session.str_usr_name);
                let mut nick_name = session.str_nick_name.clone().unwrap_or("".to_owned());
                if let Some(content) = content {
                    if let Some(remark) = &content.remark {
                        if remark.len() != 0 {
                            nick_name = format!("{} ({})",nick_name,remark);
                        }
                    }
                };
                col = col.push(
                    Button::new(
                        Column::new()
                            .push(
                                Text::new(nick_name.clone()).size(20).shaping(Shaping::Advanced)
                            ).push(
                                Text::new(session.str_content.clone().unwrap_or("".to_owned())).size(17).shaping(Shaping::Advanced)
                            )
                    ).width(Length::Fill).style(iced::theme::Button::Secondary).on_press_maybe(
                        if !self.analysis_running && !self.msg_getting {
                            Some(Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::ButtonSession(index,nick_name.clone())))
                        } else {
                            None
                        }
                    )
                )
            }
            col
        })
    }

    pub fn draw(&self) -> Container<Message> {
        Container::new(
            {
                let mut col = Column::new().spacing(5);
                match self.body {
                    Body::Session => {
                        col = col.push(
                            Scrollable::new(
                                self.draw_session().width(Length::Fill)
                            ).height(Length::Fill).id(self.session_scroll_id.clone())
                        );
                    },
                    Body::Msg => {
                        col = col.push(
                            Text::new(&self.message_title).size(20).shaping(Shaping::Advanced)
                        );
                        col = col.push(
                            Scrollable::new(
                                self.draw_message().width(Length::Fill)
                            ).height(Length::Fill).id(self.message_scroll_id.clone())
                        );
                        col = col.push(
                            Row::new().spacing(5).push(Space::with_width(5))
                                .push(
                                    Button::new("上一页").on_press_maybe(
                                        if self.msg_page == 0 {
                                            None
                                        } else {
                                            Some(Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::ButtonMsgPrev))
                                        }
                                    )
                                )
                                .push(
                                    TextInput::new("",&self.msg_page_input)
                                        .width(45).on_input(|s| Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::InputMsgPage(s)))
                                )
                                .push(
                                    Text::new(format!("总页数：{}",self.msg_list.len() / 100 + 1)).height(Length::Shrink)
                                )
                                .push(
                                    Button::new("跳转").on_press_maybe(
                                        {
                                            if let Ok(p) = self.msg_page_input.trim().parse::<usize>() {
                                                if p < 1 || (p-1) * 100 > self.msg_list.len() {
                                                    None
                                                } else {
                                                    Some(Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::ButtonMsgJumpTo(p-1)))
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                    )
                                )
                                .push(
                                    Button::new("下一页").on_press_maybe(
                                        if (self.msg_page+1) * 100 > self.msg_list.len() {
                                            None
                                        } else {
                                            Some(Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::ButtonMsgNext))
                                        }
                                    )
                                )
                                .push(
                                    Button::new("返回").on_press_maybe(
                                        if  !self.analysis_running && !self.msg_getting {
                                            Some(Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::ButtonBack))
                                        } else {
                                            None
                                        }
                                    )
                                )
                                .push(
                                    Button::new("开发用：测试silk库").on_press(
                                        Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::ButtonTestSilkSdk)
                                    )
                                )
                        );
                    },
                }
                col = set_col_with_text(
                    col, 
                    "解密文件夹", 
                    if !self.analysis_running && !self.msg_getting {
                        Some(Into::<Message>::into(AnalysisDatabaseMessage::ButtonAnalysis))
                    } else {
                        None
                    }, 
                    &self.decrypted_path, 
                    "解析", 
                    &self.analysis_database_err_msg
                );
                col = col.push(Space::with_height(5));
                col
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum AnalysisDatabaseMessage {
    UpdateAnalysisDatabase,
    ButtonAnalysis,
    ButtonSession(usize, String),
    ButtonBack,
    ButtonTestSilkSdk,
    ButtonMsgPrev,
    ButtonMsgNext,
    InputMsgPage(String),
    ButtonMsgJumpTo(usize),
    ButtonMsgPlayAudio(i64,usize),
    ButtonMsgStopAudio
}

impl Into<Message> for AnalysisDatabaseMessage {
    fn into(self) -> Message {
        Message::AnalysisDatabaseMessage(self)
    }
}

struct Conn {
    msg_conn_map: HashMap<usize, SqliteConnection>,
    macro_msg_conn: SqliteConnection,
    media_msg_conn_map: HashMap<usize, SqliteConnection>,
}

#[derive(PartialEq,Eq)]
enum Body {
    Session,
    Msg,
}

enum MsgContainerTheme {
    Left,
    Right,
}

impl iced::widget::container::StyleSheet for MsgContainerTheme {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        let mut s = style.appearance(&iced::theme::Container::Box);
        match self {
            MsgContainerTheme::Left => {
                s.background = Some(iced::Background::Color(Color::from_rgb(0.8, 0.8, 0.8)))
            },
            MsgContainerTheme::Right => {
                s.background = Some(iced::Background::Color(Color::from_rgb(0.2, 0.2, 0.8)));
                s.text_color = Some(Color::from_rgb(0.8, 0.8, 0.2));
            }
        }
        s
    }
}