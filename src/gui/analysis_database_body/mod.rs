use std::{
    collections::{HashMap, LinkedList},
    fs::read_dir,
    path::Path,
};

use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};
use cpal::Stream;
use diesel::SqliteConnection;
use iced::{
    widget::{
        scrollable::{AbsoluteOffset, Viewport},
        Button, Column, Container, Image, Row, Scrollable, Space, Text, TextInput,
    },
    Color, Length,
};
use iced_aw::DatePicker;
use iced_runtime::core::{image::Handle, text::Shaping};
use plotters::{
    backend::BitMapBackend,
    chart::ChartBuilder,
    coord::ranged1d::IntoSegmentedCoord,
    drawing::IntoDrawingArea,
    element::PathElement,
    series::{Histogram, LineSeries},
    style::{Color as pColor, ShapeStyle},
};

use self::module::module_macro_msg::{Contact, Session};

use super::{
    config_body::ConfigBody,
    gui_util::{self, set_col_with_text, set_col_with_text_input},
    Message,
};

mod module;
mod schema;

pub struct AnalysisDatabaseBody {
    decrypted_path: String,
    analysis_database_err_msg: Option<String>,
    conn: Option<Conn>,
    session_info: SessionInfo,
    msg_info: MsgInfo,
    report_info: ReportInfo,
    audio_stream: Option<(Stream, i64)>,
    body: Body,
    contact: HashMap<Option<String>, Contact>,
    chat_room_user_list: HashMap<Option<String>, Vec<String>>,
    analysis_running: bool,
    msg_getting: bool,
    wechat_path: Option<String>,
    prev_body: LinkedList<PrevBody>,
    report_image: Option<Vec<u8>>,
}

#[derive(Clone)]
struct SessionInfo {
    session_list: Vec<Session>,
    display_session_list: Vec<Session>,
    session_scroll_id: iced::widget::scrollable::Id,
    absolute_offset: AbsoluteOffset,
    search_session: String,
}

impl Default for SessionInfo {
    fn default() -> Self {
        Self {
            session_list: vec![],
            display_session_list: vec![],
            session_scroll_id: iced::widget::scrollable::Id::new("analysis_session_scroll"),
            absolute_offset: AbsoluteOffset::default(),
            search_session: String::new(),
        }
    }
}

#[derive(Clone)]
struct MsgInfo {
    msg_list: Vec<(usize, module::module_msg::Msg)>,
    message_title: String,
    message_scroll_id: iced::widget::scrollable::Id,
    msg_page: usize,
    msg_page_input: String,
    audio_err_texts: HashMap<i64, String>,
    msg_str_usr_name: Option<Option<String>>,
    absolute_offset: AbsoluteOffset,
}

impl Default for MsgInfo {
    fn default() -> Self {
        Self {
            msg_list: vec![],
            message_title: String::new(),
            message_scroll_id: iced::widget::scrollable::Id::new("analysis_message_scroll"),
            msg_page: 0,
            msg_page_input: "1".to_string(),
            audio_err_texts: HashMap::new(),
            msg_str_usr_name: None,
            absolute_offset: AbsoluteOffset::default(),
        }
    }
}

#[derive(Default)]
struct ReportInfo {
    all_msg_count: usize,
    count_by_user: Vec<(String, usize)>,
    count_by_user_top_start: String,
    count_by_user_top_end: String,
    count_by_date_order_by_date: Vec<(NaiveDate, usize)>,
    count_by_date_order_by_count: Vec<(NaiveDate, usize)>,
    count_by_date_start_show_picker: bool,
    count_by_date_start: NaiveDate,
    count_by_date_end_show_picker: bool,
    count_by_date_end: NaiveDate,
    count_by_date_top_start_show_picker: bool,
    count_by_date_top_start_date: NaiveDate,
    count_by_date_top_end_show_picker: bool,
    count_by_date_top_end_date: NaiveDate,
    count_by_date_top_start: String,
    count_by_date_top_end: String,
    count_by_date_user: Vec<(NaiveDate, String, usize)>,
    count_by_date_month: Vec<(NaiveDate, usize)>,
    count_by_date_month_user: Vec<(NaiveDate, String, usize)>,
    count_by_hour: Vec<(u32, usize)>,
    count_by_hour_user: Vec<(u32, String, usize)>,
    count_by_day: Vec<(u32, usize)>,
    count_by_month: Vec<(u32, usize)>,
    count_by_year: Vec<(i32, usize)>,
    count_by_day_user: Vec<(u32, String, usize)>,
    count_by_month_user: Vec<(u32, String, usize)>,
    count_by_year_user: Vec<(i32, String, usize)>,
    err_info: Option<String>,
}

enum PrevBody {
    Session(SessionInfo),
    Msg(MsgInfo),
}

impl AnalysisDatabaseBody {
    pub fn new() -> Self {
        Self {
            decrypted_path: String::new(),
            analysis_database_err_msg: None,
            conn: None,
            body: Body::Session,
            contact: HashMap::new(),
            analysis_running: false,
            msg_getting: false,
            wechat_path: None,
            session_info: SessionInfo::default(),
            msg_info: MsgInfo::default(),
            audio_stream: None,
            prev_body: LinkedList::new(),
            report_info: ReportInfo::default(),
            report_image: None,
            chat_room_user_list: HashMap::new(),
        }
    }

    pub fn update(
        &mut self,
        msg: AnalysisDatabaseMessage,
        config_body: &ConfigBody,
    ) -> iced::Command<Message> {
        match msg {
            AnalysisDatabaseMessage::UpdateAnalysisDatabase => {
                self.decrypted_path = config_body.decrypt_path.trim().to_owned();
                self.wechat_path = config_body.wechat_dir.to_owned();
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonAnalysis => {
                self.analysis_running = true;
                match self.get_conn() {
                    Ok(mut conn) => {
                        self.prev_body = LinkedList::new();
                        self.session_info = SessionInfo::default();
                        self.msg_info = MsgInfo::default();
                        match module::module_macro_msg::get_sessions(&mut conn.macro_msg_conn) {
                            Ok(v) => {
                                self.session_info.session_list = v;
                                self.session_info.display_session_list =
                                    self.session_info.session_list.clone();
                                self.body = Body::Session;
                                match module::module_macro_msg::get_contact(
                                    &mut conn.macro_msg_conn,
                                ) {
                                    Ok(map) => self.contact = map,
                                    Err(e) => self.analysis_database_err_msg = Some(e.to_string()),
                                }
                                match module::module_macro_msg::get_chat_room_users_map(
                                    &mut conn.macro_msg_conn,
                                ) {
                                    Ok(map) => self.chat_room_user_list = map,
                                    Err(e) => self.analysis_database_err_msg = Some(e.to_string()),
                                }
                            }
                            Err(e) => self.analysis_database_err_msg = Some(e.to_string()),
                        }
                        self.conn = Some(conn);
                    }
                    Err(r) => {
                        self.analysis_database_err_msg = Some(r.to_string());
                    }
                }
                self.analysis_running = false;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonSession(index, message_title) => {
                self.msg_getting = true;
                match module::module_msg::get_msg_by_user_name(
                    &self.session_info.display_session_list[index].str_usr_name,
                    &mut self.conn.as_mut().unwrap().msg_conn_map,
                ) {
                    Ok(s) => {
                        self.msg_info = MsgInfo::default();
                        self.msg_info.msg_list = s;
                        self.msg_info.msg_page = self.msg_info.msg_list.len() / 100;
                        self.msg_info.msg_page_input = (self.msg_info.msg_page + 1).to_string();
                        self.msg_info.message_title = message_title;
                        self.msg_getting = false;
                        self.msg_info.msg_str_usr_name = Some(
                            self.session_info.display_session_list[index]
                                .str_usr_name
                                .clone(),
                        );
                        self.body = Body::Msg;
                        self.prev_body
                            .push_back(PrevBody::Session(self.session_info.clone()));
                        return iced::widget::scrollable::snap_to::<Message>(
                            self.msg_info.message_scroll_id.clone(),
                            iced::widget::scrollable::RelativeOffset { x: 0.0, y: 1.0 },
                        );
                    }
                    Err(e) => self.analysis_database_err_msg = Some(e.to_string()),
                }
                self.msg_getting = false;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonBack => {
                if let Some(prev_body) = self.prev_body.pop_back() {
                    match prev_body {
                        PrevBody::Session(session_info) => {
                            self.session_info = session_info;
                            self.body = Body::Session;
                            iced::widget::scrollable::scroll_to(
                                self.session_info.session_scroll_id.clone(),
                                self.session_info.absolute_offset.clone(),
                            )
                        }
                        PrevBody::Msg(msg_info) => {
                            self.msg_info = msg_info;
                            self.body = Body::Msg;
                            iced::widget::scrollable::scroll_to(
                                self.msg_info.message_scroll_id.clone(),
                                self.msg_info.absolute_offset.clone(),
                            )
                        }
                    }
                } else {
                    iced::Command::none()
                }
            }
            AnalysisDatabaseMessage::ButtonMsgPrev => {
                self.msg_info.msg_page -= 1;
                self.msg_info.msg_page_input = (self.msg_info.msg_page + 1).to_string();
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonMsgNext => {
                self.msg_info.msg_page += 1;
                self.msg_info.msg_page_input = (self.msg_info.msg_page + 1).to_string();
                iced::Command::none()
            }
            AnalysisDatabaseMessage::InputMsgPage(s) => {
                self.msg_info.msg_page_input = s;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonMsgJumpTo(p) => {
                self.msg_info.msg_page = p;
                self.msg_info.msg_page_input = (self.msg_info.msg_page + 1).to_string();
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonMsgPlayAudio(id, key) => {
                if let Some(conn) = self.conn.as_mut().unwrap().media_msg_conn_map.get_mut(&key) {
                    match gui_util::play_audio(|sample_rate| {
                        module::module_media_msg::get_audio_pcm(id, sample_rate, conn)
                    }) {
                        Ok(stream) => {
                            self.audio_stream = Some((stream, id));
                        }
                        Err(e) => {
                            self.msg_info.audio_err_texts.insert(id, e.to_string());
                        }
                    }
                } else {
                    self.msg_info
                        .audio_err_texts
                        .insert(id, "未找到音频数据库".to_string());
                }
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonMsgStopAudio => {
                self.audio_stream = None;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonTestSilkSdk => {
                module::module_media_msg::test_for_silk(
                    self.conn
                        .as_mut()
                        .unwrap()
                        .media_msg_conn_map
                        .get_mut(&0)
                        .unwrap(),
                )
                .unwrap();
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonAnalyzeReport => {
                self.report_info = ReportInfo::default();
                self.report_image = None;
                let report_data = match self.body {
                    Body::Session => {
                        self.prev_body
                            .push_back(PrevBody::Session(self.session_info.clone()));
                        module::module_msg::get_report_data(
                            &None,
                            &mut self.conn.as_mut().unwrap().msg_conn_map,
                        )
                    }
                    Body::Msg => {
                        self.prev_body
                            .push_back(PrevBody::Msg(self.msg_info.clone()));
                        module::module_msg::get_report_data(
                            &self.msg_info.msg_str_usr_name,
                            &mut self.conn.as_mut().unwrap().msg_conn_map,
                        )
                    }
                    Body::Report => Err(anyhow::anyhow!("错误的页面")),
                };
                match report_data {
                    Ok(vec) => {
                        self.report_info.all_msg_count = vec.len();

                        macro_rules! build_report_data {
                            ($([$ident:ident => ($x:expr),($y:expr),($z:expr)]),+ $(,)? ) => {
                                $(let mut $ident = HashMap::new();)*
                                for data in vec {
                            let user = if data.is_sender.unwrap() == 1 {
                                None
                            } else {
                                if let Some(data_extra) = data.bytes_extra {
                                    if let Some(user_name) = data_extra.map.get(&1) {
                                        Some(user_name[0].clone())
                                    } else {
                                        Some(data.str_talker.unwrap())
                                    }
                                } else {
                                    Some(data.str_talker.unwrap())
                                }
                            };
                            let date = DateTime::from_timestamp(data.create_time.unwrap() as i64, 0).unwrap();
                            $(
                                let f = $x;
                                let key = f(&user,&date);
                                if let Some(count) = $ident.get_mut(&key) {
                                    *count += 1;
                                } else {
                                    $ident.insert(key, 1);
                                }
                            )*
                        }
                            $(
                                self.report_info.$ident = $ident.iter()
                            .map($y)
                            .collect();
                        self.report_info.$ident.sort_by($z);
                            )*
                            };
                        }

                        build_report_data!([
                            count_by_user => (
                                |user:&Option<String>,_date| user.clone()
                            ),(
                                |(k,v)| (self.get_user_name(k),*v)
                            ),(
                                |a,b| b.1.cmp(&a.1))
                            ],[
                            count_by_date_order_by_count => (
                                |_user,date:&DateTime<Utc>| date.date_naive()
                            ),(
                                |(k,v)| (*k,*v)
                            ),(
                                |a,b| b.1.cmp(&a.1))
                            ],[
                                count_by_date_order_by_date => (
                                    |_user,date:&DateTime<Utc>| date.date_naive()
                                ),(
                                    |(k,v)| (*k,*v)
                                ),(
                                    |a,b| a.0.cmp(&b.0))
                            ],[
                            count_by_date_user => (
                                    |user:&Option<String>,date:&DateTime<Utc>| (date.date_naive(),user.clone())
                                ),(
                                    |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                ),(
                                    |a,b| b.2.cmp(&a.2))
                            ],[
                            count_by_date_month => (
                                    |_user,date:&DateTime<Utc>| date.with_day(1).unwrap().date_naive()
                                ),(
                                    |(k,v)| (*k,*v)
                                ),(
                                    |a,b| b.1.cmp(&a.1))
                            ],[
                            count_by_date_month_user => (
                                    |user:&Option<String>,date:&DateTime<Utc>| (date.with_day(1).unwrap().date_naive(),user.clone())
                                ),(
                                    |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                ),(
                                    |a,b| b.2.cmp(&a.2))
                            ],[
                            count_by_hour => (
                                    |_user,date:&DateTime<Utc>| date.hour()
                                ),(
                                    |(k,v)| (*k,*v)
                                ),(
                                    |a,b| b.1.cmp(&a.1))
                            ],[
                            count_by_hour_user => (
                                    |user:&Option<String>,date:&DateTime<Utc>| (date.hour(),user.clone())
                                ),(
                                    |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                ),(
                                    |a,b| b.2.cmp(&a.2))
                            ],[
                            count_by_day => (
                                    |_user,date:&DateTime<Utc>| date.day()
                                ),(
                                    |(k,v)| (*k,*v)
                                ),(
                                    |a,b| b.1.cmp(&a.1))
                            ],[
                            count_by_day_user => (
                                    |user:&Option<String>,date:&DateTime<Utc>| (date.day(),user.clone())
                                ),(
                                    |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                ),(
                                    |a,b| b.2.cmp(&a.2))
                            ],[
                            count_by_month => (
                                    |_user,date:&DateTime<Utc>| date.month()
                                ),(
                                    |(k,v)| (*k,*v)
                                ),(
                                    |a,b| b.1.cmp(&a.1))
                            ],[
                            count_by_month_user => (
                                    |user:&Option<String>,date:&DateTime<Utc>| (date.month(),user.clone())
                                ),(
                                    |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                ),(
                                    |a,b| b.2.cmp(&a.2))
                            ],[
                            count_by_year => (
                                    |_user,date:&DateTime<Utc>| date.year()
                                ),(
                                    |(k,v)| (*k,*v)
                                ),(
                                    |a,b| b.1.cmp(&a.1))
                            ],[
                            count_by_year_user => (
                                    |user:&Option<String>,date:&DateTime<Utc>| (date.year(),user.clone())
                                ),(
                                    |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                ),(
                                    |a,b| b.2.cmp(&a.2))
                            ],
                        );
                        if self.report_info.count_by_date_order_by_date.len() != 0 {
                            self.report_info.count_by_date_start = self
                                .report_info
                                .count_by_date_order_by_date
                                .first()
                                .unwrap()
                                .0
                                .clone();
                            self.report_info.count_by_date_end = self
                                .report_info
                                .count_by_date_order_by_date
                                .last()
                                .unwrap()
                                .0
                                .clone();
                            self.report_info.count_by_date_top_start_date = self
                                .report_info
                                .count_by_date_order_by_date
                                .first()
                                .unwrap()
                                .0
                                .clone();
                            self.report_info.count_by_date_top_end_date = self
                                .report_info
                                .count_by_date_order_by_date
                                .last()
                                .unwrap()
                                .0
                                .clone();
                        }
                        self.report_info.err_info = None;
                    }
                    Err(e) => self.report_info.err_info = Some(e.to_string()),
                }
                self.body = Body::Report;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ScrollSession(viewport) => {
                self.session_info.absolute_offset = viewport.absolute_offset();
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ScrollMsg(viewport) => {
                self.msg_info.absolute_offset = viewport.absolute_offset();
                iced::Command::none()
            }
            AnalysisDatabaseMessage::InputReportCountByUserTopStart(input) => {
                if input.trim().parse::<usize>().is_ok() || input.trim().len() == 0 {
                    self.report_info.count_by_user_top_start = input;
                }
                iced::Command::none()
            }
            AnalysisDatabaseMessage::InputReportCountByUserTopEnd(input) => {
                if input.trim().parse::<usize>().is_ok() || input.trim().len() == 0 {
                    self.report_info.count_by_user_top_end = input;
                }
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonReportCountByUserTable => {
                let start = self
                    .report_info
                    .count_by_user_top_start
                    .parse::<usize>()
                    .unwrap_or(1)
                    .min(self.report_info.count_by_user.len() - 1)
                    .max(1);
                let end = self
                    .report_info
                    .count_by_user_top_end
                    .parse::<usize>()
                    .unwrap_or(self.report_info.count_by_user.len())
                    .min(self.report_info.count_by_user.len())
                    .max(start);
                let d = &self.report_info.count_by_user[(start - 1)..end];
                self.report_info.count_by_user_top_start = start.to_string();
                self.report_info.count_by_user_top_end = end.to_string();

                self.report_image = Some(Self::rgb_to_rgba(
                    &Self::get_report_histogram_image_order_by_count("不同用户的聊天记录总数", d),
                ));
                iced::Command::none()
            }
            AnalysisDatabaseMessage::DatePickerReportCountByDateStartUnderlay => {
                self.report_info.count_by_date_start_show_picker =
                    !self.report_info.count_by_date_start_show_picker;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::DatePickerReportCountByDateStartCancel => {
                if self.report_info.count_by_date_order_by_date.len() != 0 {
                    self.report_info.count_by_date_start = self
                        .report_info
                        .count_by_date_order_by_date
                        .first()
                        .unwrap()
                        .0
                        .clone()
                        .min(self.report_info.count_by_date_end);
                }
                self.report_info.count_by_date_start_show_picker = false;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::DatePickerReportCountByDateStartSubmit(date) => {
                self.report_info.count_by_date_start = date.min(self.report_info.count_by_date_end);
                self.report_info.count_by_date_start_show_picker = false;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::DatePickerReportCountByDateEndUnderlay => {
                self.report_info.count_by_date_end_show_picker =
                    !self.report_info.count_by_date_end_show_picker;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::DatePickerReportCountByDateEndCancel => {
                if self.report_info.count_by_date_order_by_date.len() != 0 {
                    self.report_info.count_by_date_end = self
                        .report_info
                        .count_by_date_order_by_date
                        .last()
                        .unwrap()
                        .0
                        .clone()
                        .max(self.report_info.count_by_date_start);
                }
                self.report_info.count_by_date_end_show_picker = false;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::DatePickerReportCountByDateEndSubmit(date) => {
                self.report_info.count_by_date_end = date.max(self.report_info.count_by_date_start);
                self.report_info.count_by_date_end_show_picker = false;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonReportCountByDateOrderByDateTable => {
                let mut data = vec![];
                for (date, count) in &self.report_info.count_by_date_order_by_date {
                    if date >= &self.report_info.count_by_date_start
                        && date <= &self.report_info.count_by_date_end
                    {
                        data.push((date.clone(), "总数".to_owned(), *count))
                    }
                }
                self.report_image = Some(Self::rgb_to_rgba(
                    &Self::get_report_line_series_image_order_by_date(
                        "不同时间的聊天记录总数",
                        &data,
                    ),
                ));
                iced::Command::none()
            }
            AnalysisDatabaseMessage::InputReportCountByDateTopStart(input) => {
                if input.trim().parse::<usize>().is_ok() || input.trim().len() == 0 {
                    self.report_info.count_by_date_top_start = input;
                }
                iced::Command::none()
            }
            AnalysisDatabaseMessage::InputReportCountByDateTopEnd(input) => {
                if input.trim().parse::<usize>().is_ok() || input.trim().len() == 0 {
                    self.report_info.count_by_date_top_end = input;
                }
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonReportCountByDateTable => {
                let mut count_by_date_order_by_count = vec![];
                for (date, count) in &self.report_info.count_by_date_order_by_count {
                    if date >= &self.report_info.count_by_date_top_start_date
                        && date <= &self.report_info.count_by_date_top_end_date
                    {
                        count_by_date_order_by_count.push((date.to_string(), *count))
                    }
                }
                let start = self
                    .report_info
                    .count_by_date_top_start
                    .parse::<usize>()
                    .unwrap_or(1)
                    .min(count_by_date_order_by_count.len() - 1)
                    .max(1);
                let end = self
                    .report_info
                    .count_by_date_top_end
                    .parse::<usize>()
                    .unwrap_or(count_by_date_order_by_count.len())
                    .min(count_by_date_order_by_count.len())
                    .max(start);
                let d = &count_by_date_order_by_count[(start - 1)..end];
                self.report_info.count_by_date_top_start = start.to_string();
                self.report_info.count_by_date_top_end = end.to_string();

                self.report_image = Some(Self::rgb_to_rgba(
                    &Self::get_report_histogram_image_order_by_count("不同用户的聊天记录总数", d),
                ));
                iced::Command::none()
            }
            AnalysisDatabaseMessage::DatePickerReportCountByDateTopStartUnderlay => {
                self.report_info.count_by_date_top_start_show_picker =
                    !self.report_info.count_by_date_top_start_show_picker;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::DatePickerReportCountByDateTopStartCancel => {
                if self.report_info.count_by_date_order_by_date.len() != 0 {
                    self.report_info.count_by_date_top_start_date = self
                        .report_info
                        .count_by_date_order_by_date
                        .first()
                        .unwrap()
                        .0
                        .clone()
                        .min(self.report_info.count_by_date_top_end_date);
                }
                self.report_info.count_by_date_top_start_show_picker = false;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::DatePickerReportCountByDateTopStartSubmit(date) => {
                self.report_info.count_by_date_top_start_date =
                    date.min(self.report_info.count_by_date_top_end_date);
                self.report_info.count_by_date_top_start_show_picker = false;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::DatePickerReportCountByDateTopEndUnderlay => {
                self.report_info.count_by_date_top_end_show_picker =
                    !self.report_info.count_by_date_top_end_show_picker;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::DatePickerReportCountByDateTopEndCancel => {
                if self.report_info.count_by_date_order_by_date.len() != 0 {
                    self.report_info.count_by_date_top_end_date = self
                        .report_info
                        .count_by_date_order_by_date
                        .last()
                        .unwrap()
                        .0
                        .clone()
                        .max(self.report_info.count_by_date_top_start_date);
                }
                self.report_info.count_by_date_top_end_show_picker = false;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::DatePickerReportCountByDateTopEndSubmit(date) => {
                self.report_info.count_by_date_top_end_date =
                    date.max(self.report_info.count_by_date_top_start_date);
                self.report_info.count_by_date_top_end_show_picker = false;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::ButtonSearchSession => {
                self.msg_getting = true;
                self.session_info.search_session =
                    self.session_info.search_session.trim().to_owned();
                let check_contact = |contact: &Contact, search_session: &String| {
                    contact
                        .user_name
                        .as_ref()
                        .unwrap_or(&"".to_owned())
                        .contains(search_session)
                        || contact
                            .alias
                            .as_ref()
                            .unwrap_or(&"".to_owned())
                            .contains(search_session)
                        || contact
                            .nick_name
                            .as_ref()
                            .unwrap_or(&"".to_owned())
                            .contains(search_session)
                        || contact
                            .remark
                            .as_ref()
                            .unwrap_or(&"".to_owned())
                            .contains(search_session)
                };
                if self.session_info.search_session.len() == 0 {
                    self.session_info.display_session_list = self.session_info.session_list.clone();
                } else {
                    self.session_info.display_session_list = vec![];
                    for session in &self.session_info.session_list {
                        if let Some(contact) = self.contact.get(&session.str_usr_name) {
                            if check_contact(contact, &self.session_info.search_session) {
                                self.session_info.display_session_list.push(session.clone());
                            } else {
                                if contact.r#type == Some(2) {
                                    if let Some(user_list) =
                                        self.chat_room_user_list.get(&contact.user_name)
                                    {
                                        for user in user_list {
                                            if let Some(user_contact) =
                                                self.contact.get(&Some(user.to_owned()))
                                            {
                                                if check_contact(
                                                    user_contact,
                                                    &self.session_info.search_session,
                                                ) {
                                                    self.session_info
                                                        .display_session_list
                                                        .push(session.clone());
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self.msg_getting = false;
                iced::Command::none()
            }
            AnalysisDatabaseMessage::InputSearchSession(input) => {
                self.session_info.search_session = input;
                iced::Command::none()
            }
        }
    }

    fn get_report_line_series_image_order_by_date(
        title: &str,
        d: &[(NaiveDate, String, usize)],
    ) -> Vec<u8> {
        let mut vec = vec![0u8; 1024 * 768 * 3];
        {
            let root = BitMapBackend::with_buffer(vec.as_mut(), (1024, 768)).into_drawing_area();
            root.fill(&plotters::style::WHITE).unwrap();
            let mut chart = ChartBuilder::on(&root)
                .x_label_area_size(35)
                .y_label_area_size(40)
                .margin(5)
                .caption(title, ("楷体", 50.0))
                .build_cartesian_2d(
                    d.first()
                        .unwrap_or(&(NaiveDate::default(), String::default(), usize::default()))
                        .0
                        ..d.last()
                            .unwrap_or(&(NaiveDate::default(), String::default(), usize::default()))
                            .0,
                    0..d.iter().map(|(_, _, count)| *count).max().unwrap_or(0),
                )
                .unwrap();
            chart
                .configure_mesh()
                .disable_x_mesh()
                .bold_line_style(plotters::style::WHITE.mix(0.3))
                .y_desc("总数")
                .x_desc("排名")
                .axis_desc_style(("楷体", 15))
                .draw()
                .unwrap();
            let mut map: HashMap<String, Vec<(NaiveDate, usize)>> = HashMap::new();
            for (date, name, count) in d.iter() {
                if let Some(v) = map.get_mut(name) {
                    v.push((date.clone(), *count));
                } else {
                    map.insert(name.clone(), vec![(date.clone(), *count)]);
                }
            }
            let mut order_vec = vec![];
            for (name, vec) in &map {
                let a: usize = vec.iter().map(|(_, count)| *count).sum();
                order_vec.push((name.clone(), a));
            }
            order_vec.sort_by(|a, b| a.1.cmp(&b.1));
            for (index, (name, count)) in order_vec.iter().enumerate() {
                chart
                    .draw_series(LineSeries::new(
                        map.get(name)
                            .unwrap()
                            .iter()
                            .map(|(date, count)| (date.clone(), *count)),
                        plotters::style::RED
                            .mix(0.3 + index as f64 * (0.5 / d.len() as f64))
                            .filled(),
                    ))
                    .unwrap()
                    .label(format!("{} {}", name, count))
                    .legend(move |(x, y)| {
                        PathElement::new(
                            vec![(x, y), (x + 20, y)],
                            &plotters::style::RED.mix(0.3 + index as f64 * (0.5 / d.len() as f64)),
                        )
                    });
            }
            chart
                .configure_series_labels()
                .border_style(ShapeStyle {
                    color: plotters::style::BLACK.into(),
                    filled: false,
                    stroke_width: 0,
                })
                .label_font(("楷体", 20))
                .draw()
                .unwrap();
            root.present().unwrap();
        }
        vec
    }

    fn get_report_histogram_image_order_by_count(title: &str, d: &[(String, usize)]) -> Vec<u8> {
        let mut vec = vec![0u8; 1024 * 768 * 3];
        {
            let root = BitMapBackend::with_buffer(vec.as_mut(), (1024, 768)).into_drawing_area();
            root.fill(&plotters::style::WHITE).unwrap();
            let mut chart = ChartBuilder::on(&root)
                .x_label_area_size(35)
                .y_label_area_size(40)
                .margin(5)
                .caption(title, ("楷体", 50.0))
                .build_cartesian_2d((1..d.len()).into_segmented(), 0..d[0].1)
                .unwrap();
            chart
                .configure_mesh()
                .disable_x_mesh()
                .bold_line_style(plotters::style::WHITE.mix(0.3))
                .y_desc("总数")
                .x_desc("排名")
                .axis_desc_style(("楷体", 15))
                .draw()
                .unwrap();
            for (index, (name, count)) in d.iter().enumerate() {
                chart
                    .draw_series(
                        Histogram::vertical(&chart)
                            .style(
                                plotters::style::RED
                                    .mix(0.3 + index as f64 * (0.5 / d.len() as f64))
                                    .filled(),
                            )
                            .margin(20)
                            .data(vec![(index + 1, *count)]),
                    )
                    .unwrap()
                    .label(format!("{} {}", name, count));
            }
            chart
                .configure_series_labels()
                .border_style(ShapeStyle {
                    color: plotters::style::BLACK.into(),
                    filled: false,
                    stroke_width: 0,
                })
                .label_font(("楷体", 20))
                .draw()
                .unwrap();
            root.present().unwrap();
        }
        vec
    }

    fn rgb_to_rgba(mut data: &[u8]) -> Vec<u8> {
        let mut vec = vec![];
        loop {
            let (l, r) = data.split_at(3);
            vec.append(&mut l.to_vec());
            vec.push(255);
            if r.len() == 0 {
                break;
            } else {
                data = r;
            }
        }
        vec
    }

    fn get_user_name(&self, user: &Option<String>) -> String {
        let user = if let Some(user) = user {
            if let Some(contact) = self.contact.get(&Some(user.to_owned())) {
                let mut text = contact
                    .nick_name
                    .as_ref()
                    .cloned()
                    .unwrap_or("".to_string());
                if let Some(remark) = &contact.remark {
                    if remark.len() > 0 {
                        text = format!("{}({})", text, remark);
                    }
                }
                text
            } else {
                "未知用户".to_string()
            }
        } else {
            "本人".to_string()
        };
        user
    }

    fn get_conn(&self) -> Result<Conn, anyhow::Error> {
        let decrypted_path = Path::new(self.decrypted_path.trim());
        let mut msg_conn_map = HashMap::new();
        let mut media_msg_conn_map = HashMap::new();
        let mut macro_msg_conn = None;
        for entry in read_dir(decrypted_path)? {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with("decrypted_MSG") && filename.ends_with(".db") {
                        if let Ok(index) = filename
                            .replace("decrypted_MSG", "")
                            .replace(".db", "")
                            .parse::<usize>()
                        {
                            msg_conn_map.insert(index, module::get_conn(entry.path().display())?);
                        }
                    } else if filename.starts_with("decrypted_MediaMSG")
                        && filename.ends_with(".db")
                    {
                        if let Ok(index) = filename
                            .replace("decrypted_MediaMSG", "")
                            .replace(".db", "")
                            .parse::<usize>()
                        {
                            media_msg_conn_map
                                .insert(index, module::get_conn(entry.path().display())?);
                        }
                    } else if filename == "decrypted_MicroMsg.db" {
                        macro_msg_conn = Some(module::get_conn(entry.path().display())?);
                    }
                }
            }
        }
        Ok(Conn {
            msg_conn_map,
            media_msg_conn_map,
            macro_msg_conn: macro_msg_conn.ok_or(anyhow::anyhow!("未找到MicroMsg数据库"))?,
        })
    }
    fn draw_message(&self) -> Container<Message> {
        Container::new({
            let mut col = Column::new().spacing(10);
            let list = if (self.msg_info.msg_page + 1) * 100 > self.msg_info.msg_list.len() {
                &self.msg_info.msg_list[self.msg_info.msg_page * 100..]
            } else {
                &self.msg_info.msg_list
                    [self.msg_info.msg_page * 100..(self.msg_info.msg_page + 1) * 100]
            };
            for (index, msg) in list {
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
                                                Container::new(
                                                    Button::new(
                                                        Image::new(Handle::from_memory(image.clone()))
                                                    )
                                                    .padding(0)
                                                    .on_press(Message::OpenImage(image))
                                                )
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
                                                if let Some(s) = self.msg_info.audio_err_texts.get(&id) {
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

    fn draw_report(&self) -> Container<Message> {
        if let Some(err) = self.report_info.err_info.as_ref() {
            Container::new(
                Text::new(err).style(iced::theme::Text::Color(Color::from_rgb(1.0, 0.0, 0.0))),
            )
        } else {
            Container::new({
                let mut col = Column::new();
                col = col.push(Text::new(format!(
                    "总数：{}",
                    self.report_info.all_msg_count
                )));
                col =
                    col.push(
                        Row::new()
                            .push(Text::new("不同用户的聊天记录总数"))
                            .push(Text::new("前"))
                            .push(
                                TextInput::new("", &self.report_info.count_by_user_top_start)
                                    .on_input(|input| {
                                        Message::AnalysisDatabaseMessage(
                                            AnalysisDatabaseMessage::InputReportCountByUserTopStart(
                                                input,
                                            ),
                                        )
                                    }),
                            )
                            .push(Text::new("-"))
                            .push(
                                TextInput::new("", &self.report_info.count_by_user_top_end)
                                    .on_input(|input| {
                                        Message::AnalysisDatabaseMessage(
                                            AnalysisDatabaseMessage::InputReportCountByUserTopEnd(
                                                input,
                                            ),
                                        )
                                    }),
                            )
                            .push(Button::new("生成图表").on_press(
                                Message::AnalysisDatabaseMessage(
                                    AnalysisDatabaseMessage::ButtonReportCountByUserTable,
                                ),
                            )),
                    ).push(
                        Row::new()
                            .push(Text::new("不同时间的聊天记录总数"))
                            .push(Text::new("日期"))
                            .push(
                                Container::new(
                                    Row::new()
                                    .push(
                                        DatePicker::new(
                                            self.report_info.count_by_date_start_show_picker,
                                            self.report_info.count_by_date_start,
                                            Button::new(Text::new(self.report_info.count_by_date_start.to_string())).on_press(Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::DatePickerReportCountByDateStartUnderlay)),
                                            Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::DatePickerReportCountByDateStartCancel),
                                            |date| Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::DatePickerReportCountByDateStartSubmit(date.into())),
                                        )
                                    )
                                    .push(Text::new("-"))
                                    .push(
                                        DatePicker::new(
                                            self.report_info.count_by_date_end_show_picker,
                                            self.report_info.count_by_date_end,
                                            Button::new(Text::new(self.report_info.count_by_date_end.to_string())).on_press(Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::DatePickerReportCountByDateEndUnderlay)),
                                            Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::DatePickerReportCountByDateEndCancel),
                                            |date| Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::DatePickerReportCountByDateEndSubmit(date.into())),
                                        )
                                    )
                                ).width(Length::Fill)
                            )
                            .push(Button::new("生成图表").on_press(
                                Message::AnalysisDatabaseMessage(
                                    AnalysisDatabaseMessage::ButtonReportCountByDateOrderByDateTable,
                                ),
                            )),
                    ).push(
                        Row::new()
                            .push(Text::new("不同时间的聊天记录总数"))

                            .push(Text::new("日期"))
                            .push(
                                Container::new(
                                    Row::new()
                                    .push(
                                        DatePicker::new(
                                            self.report_info.count_by_date_top_start_show_picker,
                                            self.report_info.count_by_date_top_start_date,
                                            Button::new(Text::new(self.report_info.count_by_date_top_start_date.to_string())).on_press(Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::DatePickerReportCountByDateTopStartUnderlay)),
                                            Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::DatePickerReportCountByDateTopStartCancel),
                                            |date| Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::DatePickerReportCountByDateTopStartSubmit(date.into())),
                                        )
                                    )
                                    .push(Text::new("-"))
                                    .push(
                                        DatePicker::new(
                                            self.report_info.count_by_date_top_end_show_picker,
                                            self.report_info.count_by_date_top_end_date,
                                            Button::new(Text::new(self.report_info.count_by_date_top_end_date.to_string())).on_press(Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::DatePickerReportCountByDateTopEndUnderlay)),
                                            Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::DatePickerReportCountByDateTopEndCancel),
                                            |date| Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::DatePickerReportCountByDateTopEndSubmit(date.into()))
                                        )
                                    )
                                )
                            ).push(Text::new("前"))
                            .push(
                                TextInput::new("", &self.report_info.count_by_date_top_start)
                                    .on_input(|input| {
                                        Message::AnalysisDatabaseMessage(
                                            AnalysisDatabaseMessage::InputReportCountByDateTopStart(
                                                input,
                                            ),
                                        )
                                    }),
                            )
                            .push(Text::new("-"))
                            .push(
                                TextInput::new("", &self.report_info.count_by_date_top_end)
                                    .on_input(|input| {
                                        Message::AnalysisDatabaseMessage(
                                            AnalysisDatabaseMessage::InputReportCountByDateTopEnd(
                                                input,
                                            ),
                                        )
                                    }),
                            )
                            .push(Button::new("生成图表").on_press(
                                Message::AnalysisDatabaseMessage(
                                    AnalysisDatabaseMessage::ButtonReportCountByDateTable,
                                ),
                            )),
                    );
                if let Some(image) = &self.report_image {
                    col = col.push(Image::new(Handle::from_pixels(1024, 768, image.clone())));
                }
                col
            })
        }
    }

    fn draw_session(&self) -> Container<Message> {
        Container::new({
            let mut col = Column::new();
            for (index, session) in self.session_info.display_session_list.iter().enumerate() {
                let content = self.contact.get(&session.str_usr_name);
                let mut nick_name = session.str_nick_name.clone().unwrap_or("".to_owned());
                if let Some(content) = content {
                    if let Some(remark) = &content.remark {
                        if remark.len() != 0 {
                            nick_name = format!("{} ({})", nick_name, remark);
                        }
                    }
                };
                col = col.push(
                    Button::new(
                        Column::new()
                            .push(
                                Text::new(nick_name.clone())
                                    .size(20)
                                    .shaping(Shaping::Advanced),
                            )
                            .push(
                                Text::new(session.str_content.clone().unwrap_or("".to_owned()))
                                    .size(17)
                                    .shaping(Shaping::Advanced),
                            ),
                    )
                    .width(Length::Fill)
                    .style(iced::theme::Button::Secondary)
                    .on_press_maybe(
                        if !self.analysis_running && !self.msg_getting {
                            Some(Message::AnalysisDatabaseMessage(
                                AnalysisDatabaseMessage::ButtonSession(index, nick_name.clone()),
                            ))
                        } else {
                            None
                        },
                    ),
                )
            }
            col
        })
    }

    pub fn draw(&self) -> Container<Message> {
        Container::new({
            let mut col = Column::new().spacing(5);
            match self.body {
                Body::Session => {
                    col = col.push(
                        Scrollable::new(self.draw_session().width(Length::Fill))
                            .height(Length::Fill)
                            .on_scroll(|viewport| {
                                Message::AnalysisDatabaseMessage(
                                    AnalysisDatabaseMessage::ScrollSession(viewport),
                                )
                            })
                            .id(self.session_info.session_scroll_id.clone()),
                    );
                    if self.session_info.session_list.len() != 0 {
                        col = set_col_with_text_input(
                            col,
                            "搜索",
                            |input| {
                                Message::AnalysisDatabaseMessage(
                                    AnalysisDatabaseMessage::InputSearchSession(input),
                                )
                            },
                            if !self.analysis_running && !self.msg_getting {
                                Some(Into::<Message>::into(
                                    AnalysisDatabaseMessage::ButtonSearchSession,
                                ))
                            } else {
                                None
                            },
                            &self.session_info.search_session,
                            "搜索",
                            &None::<String>,
                        );
                    }
                }
                Body::Msg => {
                    col = col.push(
                        Text::new(&self.msg_info.message_title)
                            .size(20)
                            .shaping(Shaping::Advanced),
                    );
                    col = col.push(
                        Scrollable::new(self.draw_message().width(Length::Fill))
                            .height(Length::Fill)
                            .on_scroll(|viewport| {
                                Message::AnalysisDatabaseMessage(
                                    AnalysisDatabaseMessage::ScrollMsg(viewport),
                                )
                            })
                            .id(self.msg_info.message_scroll_id.clone()),
                    );
                    col = col.push({
                        let mut row = Row::new()
                            .spacing(5)
                            .push(Space::with_width(5))
                            .push(Button::new("上一页").on_press_maybe(
                                if self.msg_info.msg_page == 0 {
                                    None
                                } else {
                                    Some(Message::AnalysisDatabaseMessage(
                                        AnalysisDatabaseMessage::ButtonMsgPrev,
                                    ))
                                },
                            ))
                            .push(
                                TextInput::new("", &self.msg_info.msg_page_input)
                                    .width(45)
                                    .on_input(|s| {
                                        Message::AnalysisDatabaseMessage(
                                            AnalysisDatabaseMessage::InputMsgPage(s),
                                        )
                                    }),
                            )
                            .push(
                                Text::new(format!(
                                    "总页数：{}",
                                    self.msg_info.msg_list.len() / 100 + 1
                                ))
                                .height(Length::Shrink),
                            )
                            .push(Button::new("跳转").on_press_maybe({
                                if let Ok(p) = self.msg_info.msg_page_input.trim().parse::<usize>()
                                {
                                    if p < 1 || (p - 1) * 100 > self.msg_info.msg_list.len() {
                                        None
                                    } else {
                                        Some(Message::AnalysisDatabaseMessage(
                                            AnalysisDatabaseMessage::ButtonMsgJumpTo(p - 1),
                                        ))
                                    }
                                } else {
                                    None
                                }
                            }))
                            .push(Button::new("下一页").on_press_maybe(
                                if (self.msg_info.msg_page + 1) * 100 > self.msg_info.msg_list.len()
                                {
                                    None
                                } else {
                                    Some(Message::AnalysisDatabaseMessage(
                                        AnalysisDatabaseMessage::ButtonMsgNext,
                                    ))
                                },
                            ))
                            .push(Button::new("返回").on_press_maybe(
                                if !self.analysis_running && !self.msg_getting {
                                    Some(Message::AnalysisDatabaseMessage(
                                        AnalysisDatabaseMessage::ButtonBack,
                                    ))
                                } else {
                                    None
                                },
                            ))
                            .push(Button::new("数据图表").on_press_maybe(
                                if !self.analysis_running && !self.msg_getting {
                                    Some(Message::AnalysisDatabaseMessage(
                                        AnalysisDatabaseMessage::ButtonAnalyzeReport,
                                    ))
                                } else {
                                    None
                                },
                            ));
                        #[cfg(feature = "dev")]
                        {
                            row = row.push(Button::new("开发用：测试silk库").on_press(
                                Message::AnalysisDatabaseMessage(
                                    AnalysisDatabaseMessage::ButtonTestSilkSdk,
                                ),
                            ));
                        }
                        row
                    });
                }
                Body::Report => {
                    col = col.push(
                        Scrollable::new(self.draw_report().width(Length::Fill))
                            .height(Length::Fill),
                    );

                    col = col.push({
                        let row = Row::new().spacing(5).push(Space::with_width(5)).push(
                            Button::new("返回").on_press(Message::AnalysisDatabaseMessage(
                                AnalysisDatabaseMessage::ButtonBack,
                            )),
                        );
                        row
                    });
                }
            }
            col = set_col_with_text(
                col,
                "解密文件夹",
                if !self.analysis_running && !self.msg_getting {
                    Some(Into::<Message>::into(
                        AnalysisDatabaseMessage::ButtonAnalysis,
                    ))
                } else {
                    None
                },
                &self.decrypted_path,
                "解析",
                &self.analysis_database_err_msg,
            );
            col = col.push(Space::with_height(5));
            col
        })
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
    ButtonMsgPlayAudio(i64, usize),
    ButtonMsgStopAudio,
    ButtonAnalyzeReport,
    ButtonSearchSession,
    InputSearchSession(String),
    ScrollSession(Viewport),
    ScrollMsg(Viewport),
    InputReportCountByUserTopStart(String),
    InputReportCountByUserTopEnd(String),
    ButtonReportCountByUserTable,
    DatePickerReportCountByDateStartUnderlay,
    DatePickerReportCountByDateStartCancel,
    DatePickerReportCountByDateStartSubmit(NaiveDate),
    DatePickerReportCountByDateEndUnderlay,
    DatePickerReportCountByDateEndCancel,
    DatePickerReportCountByDateEndSubmit(NaiveDate),
    ButtonReportCountByDateOrderByDateTable,
    DatePickerReportCountByDateTopStartUnderlay,
    DatePickerReportCountByDateTopStartCancel,
    DatePickerReportCountByDateTopStartSubmit(NaiveDate),
    DatePickerReportCountByDateTopEndUnderlay,
    DatePickerReportCountByDateTopEndCancel,
    DatePickerReportCountByDateTopEndSubmit(NaiveDate),
    InputReportCountByDateTopStart(String),
    InputReportCountByDateTopEnd(String),
    ButtonReportCountByDateTable,
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

#[derive(PartialEq, Eq)]
enum Body {
    Session,
    Msg,
    Report,
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
            }
            MsgContainerTheme::Right => {
                s.background = Some(iced::Background::Color(Color::from_rgb(0.2, 0.2, 0.8)));
                s.text_color = Some(Color::from_rgb(0.8, 0.8, 0.2));
            }
        }
        s
    }
}
