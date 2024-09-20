use std::collections::HashMap;
use std::hash::Hash;

use chrono::{NaiveDate, Weekday};
use iced::{
    widget::{image::Handle, Button, Column, Container, Image, PickList, Row, Text, TextInput},
    Color, Length,
};
use iced_aw::DatePicker;
use jieba_rs::{Jieba, KeywordExtract, KeywordExtractConfig, TextRank, TfIdf};
use plotters::{
    backend::BitMapBackend,
    chart::ChartBuilder,
    coord::ranged1d::{IntoSegmentedCoord, SegmentValue},
    drawing::IntoDrawingArea,
    element::PathElement,
    series::{Histogram, LineSeries},
    style::{Color as PlottersColor, RGBColor, RGBAColor, ShapeStyle},
};

use crate::gui::Message;

use super::{module::module_macro_msg::Contact, AnalysisDatabaseMessage, MsgInfo};

const REPORT_IMAGE_WIDTH: u32 = 1024;
const REPORT_IMAGE_HEIGHT: u32 = 768;

#[derive(Default)]
pub struct ReportInfo {
    pub all_msg_count: usize,
    pub user_select_for_date_options: Vec<UserForSelect>,
    pub user_select_for_date_selected: Option<UserForSelect>,
    pub user_select_for_jieba_options: Vec<UserForSelect>,
    pub user_select_for_jieba_selected: Option<UserForSelect>,
    pub count_by_user: Vec<(UserForSelect, usize)>,
    pub count_by_user_top_start: String,
    pub count_by_user_top_end: String,
    pub count_by_date_order_by_date: Vec<(NaiveDate, usize)>,
    pub count_by_date_order_by_count: Vec<(NaiveDate, usize)>,
    pub count_by_date_start_show_picker: bool,
    pub count_by_date_start: NaiveDate,
    pub count_by_date_end_show_picker: bool,
    pub count_by_date_end: NaiveDate,
    pub count_by_date_top_start_show_picker: bool,
    pub count_by_date_top_start_date: NaiveDate,
    pub count_by_date_top_end_show_picker: bool,
    pub count_by_date_top_end_date: NaiveDate,
    pub count_by_date_top_start: String,
    pub count_by_date_top_end: String,
    pub count_by_date_user: Vec<(NaiveDate, UserForSelect, usize)>,
    pub count_by_date_month: Vec<(NaiveDate, usize)>,
    pub count_by_date_month_user: Vec<(NaiveDate, UserForSelect, usize)>,
    pub count_by_hour: Vec<(u32, usize)>,
    pub count_by_hour_user: Vec<(u32, UserForSelect, usize)>,
    pub count_by_day: Vec<(u32, usize)>,
    pub count_by_month: Vec<(u32, usize)>,
    pub count_by_year: Vec<(i32, usize)>,
    pub count_by_day_user: Vec<(u32, UserForSelect, usize)>,
    pub count_by_month_user: Vec<(u32, UserForSelect, usize)>,
    pub count_by_year_user: Vec<(i32, UserForSelect, usize)>,
    pub count_by_weekday: Vec<(Weekday, usize)>,
    pub count_by_weekday_user: Vec<(Weekday, UserForSelect, usize)>,
    pub err_info: Option<String>,
    pub report_image: Option<Vec<u8>>,
}

#[derive(Clone, Eq, Hash, Debug)]
pub struct UserForSelect {
    id: Option<String>,
    name: String,
}

impl UserForSelect {
    fn new(id: Option<String>, name: String) -> Self {
        Self { id, name }
    }
}

impl ToString for UserForSelect {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl PartialEq for UserForSelect {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl ReportInfo {
    pub fn update(
        &mut self,
        msg: AnalysisDatabaseReportMessage,
        msg_info: &MsgInfo,
        contact: &HashMap<Option<String>, Contact>,
        theme: &iced::theme::Theme,
    ) -> iced::Task<Message> {
        match msg {
            AnalysisDatabaseReportMessage::InputReportCountByUserTopStart(input) => {
                if input.trim().parse::<usize>().is_ok() || input.trim().len() == 0 {
                    self.count_by_user_top_start = input;
                }
            }
            AnalysisDatabaseReportMessage::InputReportCountByUserTopEnd(input) => {
                if input.trim().parse::<usize>().is_ok() || input.trim().len() == 0 {
                    self.count_by_user_top_end = input;
                }
            }
            AnalysisDatabaseReportMessage::ButtonReportCountByUserTable => {
                let start = self
                    .count_by_user_top_start
                    .parse::<usize>()
                    .unwrap_or(1)
                    .min(self.count_by_user.len())
                    .max(1);
                let end = self
                    .count_by_user_top_end
                    .parse::<usize>()
                    .unwrap_or(self.count_by_user.len())
                    .min(self.count_by_user.len())
                    .max(start);
                let d = &self.count_by_user[(start - 1)..end];
                self.count_by_user_top_start = start.to_string();
                self.count_by_user_top_end = end.to_string();
                self.report_image = Some(Self::rgb_to_rgba(&Self::get_report_histogram_image(
                    "不同用户的聊天记录总数",
                    "总数",
                    "总数",
                    |index| match index {
                        SegmentValue::Exact(index) => index.to_string(),
                        SegmentValue::CenterOf(index) => index.to_string(),
                        SegmentValue::Last => "".to_string(),
                    },
                    d,
                    theme,
                )));
            }
            AnalysisDatabaseReportMessage::DatePickerReportCountByDateStartUnderlay => {
                self.count_by_date_start_show_picker = !self.count_by_date_start_show_picker;
            }
            AnalysisDatabaseReportMessage::DatePickerReportCountByDateStartCancel => {
                if self.count_by_date_order_by_date.len() != 0 {
                    self.count_by_date_start = self
                        .count_by_date_order_by_date
                        .first()
                        .unwrap()
                        .0
                        .clone()
                        .min(self.count_by_date_end);
                }
                self.count_by_date_start_show_picker = false;
            }
            AnalysisDatabaseReportMessage::DatePickerReportCountByDateStartSubmit(date) => {
                self.count_by_date_start = date.min(self.count_by_date_end);
                self.count_by_date_start_show_picker = false;
            }
            AnalysisDatabaseReportMessage::DatePickerReportCountByDateEndUnderlay => {
                self.count_by_date_end_show_picker = !self.count_by_date_end_show_picker;
            }
            AnalysisDatabaseReportMessage::DatePickerReportCountByDateEndCancel => {
                if self.count_by_date_order_by_date.len() != 0 {
                    self.count_by_date_end = self
                        .count_by_date_order_by_date
                        .last()
                        .unwrap()
                        .0
                        .clone()
                        .max(self.count_by_date_start);
                }
                self.count_by_date_end_show_picker = false;
            }
            AnalysisDatabaseReportMessage::DatePickerReportCountByDateEndSubmit(date) => {
                self.count_by_date_end = date.max(self.count_by_date_start);
                self.count_by_date_end_show_picker = false;
            }
            AnalysisDatabaseReportMessage::ButtonReportCountByDateOrderByDateTable => {
                let mut data = vec![];
                for (date, count) in &self.count_by_date_order_by_date {
                    if date >= &self.count_by_date_start && date <= &self.count_by_date_end {
                        data.push((date.clone(), "总数".to_owned(), *count))
                    }
                }
                self.report_image = Some(Self::rgb_to_rgba(
                    &Self::get_report_line_series_image_order_by_date(
                        "不同时间的聊天记录总数",
                        &data,
                        theme
                    ),
                ));
            }
            AnalysisDatabaseReportMessage::InputReportCountByDateTopStart(input) => {
                if input.trim().parse::<usize>().is_ok() || input.trim().len() == 0 {
                    self.count_by_date_top_start = input;
                }
            }
            AnalysisDatabaseReportMessage::InputReportCountByDateTopEnd(input) => {
                if input.trim().parse::<usize>().is_ok() || input.trim().len() == 0 {
                    self.count_by_date_top_end = input;
                }
            }
            AnalysisDatabaseReportMessage::ButtonReportCountByDateTable => {
                let mut count_by_date_order_by_count = vec![];
                for (date, count) in &self.count_by_date_order_by_count {
                    if date >= &self.count_by_date_top_start_date
                        && date <= &self.count_by_date_top_end_date
                    {
                        count_by_date_order_by_count.push((date.to_string(), *count))
                    }
                }
                let start = self
                    .count_by_date_top_start
                    .parse::<usize>()
                    .unwrap_or(1)
                    .min(count_by_date_order_by_count.len())
                    .max(1);
                let end = self
                    .count_by_date_top_end
                    .parse::<usize>()
                    .unwrap_or(count_by_date_order_by_count.len())
                    .min(count_by_date_order_by_count.len())
                    .max(start);
                let d = &count_by_date_order_by_count[(start - 1)..end];
                self.count_by_date_top_start = start.to_string();
                self.count_by_date_top_end = end.to_string();

                self.report_image = Some(Self::rgb_to_rgba(&Self::get_report_histogram_image(
                    "不同用户的聊天记录总数",
                    "总数",
                    "总数",
                    |index| match index {
                        SegmentValue::Exact(index) => index.to_string(),
                        SegmentValue::CenterOf(index) => index.to_string(),
                        SegmentValue::Last => "".to_string(),
                    },
                    d,
                    theme,
                )));
            }
            AnalysisDatabaseReportMessage::DatePickerReportCountByDateTopStartUnderlay => {
                self.count_by_date_top_start_show_picker =
                    !self.count_by_date_top_start_show_picker;
            }
            AnalysisDatabaseReportMessage::DatePickerReportCountByDateTopStartCancel => {
                if self.count_by_date_order_by_date.len() != 0 {
                    self.count_by_date_top_start_date = self
                        .count_by_date_order_by_date
                        .first()
                        .unwrap()
                        .0
                        .clone()
                        .min(self.count_by_date_top_end_date);
                }
                self.count_by_date_top_start_show_picker = false;
            }
            AnalysisDatabaseReportMessage::DatePickerReportCountByDateTopStartSubmit(date) => {
                self.count_by_date_top_start_date = date.min(self.count_by_date_top_end_date);
                self.count_by_date_top_start_show_picker = false;
            }
            AnalysisDatabaseReportMessage::DatePickerReportCountByDateTopEndUnderlay => {
                self.count_by_date_top_end_show_picker = !self.count_by_date_top_end_show_picker;
            }
            AnalysisDatabaseReportMessage::DatePickerReportCountByDateTopEndCancel => {
                if self.count_by_date_order_by_date.len() != 0 {
                    self.count_by_date_top_end_date = self
                        .count_by_date_order_by_date
                        .last()
                        .unwrap()
                        .0
                        .clone()
                        .max(self.count_by_date_top_start_date);
                }
                self.count_by_date_top_end_show_picker = false;
            }
            AnalysisDatabaseReportMessage::DatePickerReportCountByDateTopEndSubmit(date) => {
                self.count_by_date_top_end_date = date.max(self.count_by_date_top_start_date);
                self.count_by_date_top_end_show_picker = false;
            }
            AnalysisDatabaseReportMessage::ButtonReportCountByMonthTable => {
                let mut d = vec![];
                if let Some(user) = &self.user_select_for_date_selected {
                    d = self
                        .count_by_month_user
                        .iter()
                        .filter(|(_, u, _)| u == user)
                        .map(|(m, _, c)| (*m, *c))
                        .collect()
                };
                let d = if self.user_select_for_date_selected.is_some() {
                    d.iter()
                } else {
                    self.count_by_month.iter()
                }
                .map(|(month, count)| (format!("{}月", month), *count))
                .collect::<Vec<_>>();
                self.report_image = Some(Self::rgb_to_rgba(&Self::get_report_histogram_image(
                    "以月份为维度统计总数",
                    "月份",
                    "总数",
                    |index| match index {
                        SegmentValue::Exact(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::CenterOf(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::Last => "".to_string(),
                    },
                    &d,
                    theme,
                )));
            }
            AnalysisDatabaseReportMessage::ButtonReportCountByDayTable => {
                let mut d = vec![];
                if let Some(user) = &self.user_select_for_date_selected {
                    d = self
                        .count_by_day_user
                        .iter()
                        .filter(|(_, u, _)| u == user)
                        .map(|(d, _, c)| (*d, *c))
                        .collect()
                };
                let d = if self.user_select_for_date_selected.is_some() {
                    d.iter()
                } else {
                    self.count_by_day.iter()
                }
                .map(|(day, count)| (format!("{}日", day), *count))
                .collect::<Vec<_>>();
                self.report_image = Some(Self::rgb_to_rgba(&Self::get_report_histogram_image(
                    "以日期为维度统计总数",
                    "日期",
                    "总数",
                    |index| match index {
                        SegmentValue::Exact(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::CenterOf(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::Last => "".to_string(),
                    },
                    &d,
                    theme,
                )));
            }
            AnalysisDatabaseReportMessage::ButtonReportCountByWeekdayTable => {
                let mut d = vec![];
                if let Some(user) = &self.user_select_for_date_selected {
                    d = self
                        .count_by_weekday_user
                        .iter()
                        .filter(|(_, u, _)| u == user)
                        .map(|(w, _, c)| (*w, *c))
                        .collect()
                };
                let d = if self.user_select_for_date_selected.is_some() {
                    d.iter()
                } else {
                    self.count_by_weekday.iter()
                }
                .map(|(weekday, count)| (weekday.to_string(), *count))
                .collect::<Vec<_>>();
                self.report_image = Some(Self::rgb_to_rgba(&Self::get_report_histogram_image(
                    "以星期为维度统计总数",
                    "星期",
                    "总数",
                    |index| match index {
                        SegmentValue::Exact(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::CenterOf(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::Last => "".to_string(),
                    },
                    &d,
                    theme,
                )));
            }
            AnalysisDatabaseReportMessage::ButtonReportCountByHourTable => {
                let mut d = vec![];
                if let Some(user) = &self.user_select_for_date_selected {
                    d = self
                        .count_by_hour_user
                        .iter()
                        .filter(|(_, u, _)| u == user)
                        .map(|(h, _, c)| (*h, *c))
                        .collect()
                };
                let d = if self.user_select_for_date_selected.is_some() {
                    d.iter()
                } else {
                    self.count_by_hour.iter()
                }
                .map(|(hour, count)| (format!("{}时", hour), *count))
                .collect::<Vec<_>>();
                self.report_image = Some(Self::rgb_to_rgba(&Self::get_report_histogram_image(
                    "以小时为维度统计总数",
                    "小时",
                    "总数",
                    |index| match index {
                        SegmentValue::Exact(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::CenterOf(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::Last => "".to_string(),
                    },
                    &d,
                    theme,
                )));
            }
            AnalysisDatabaseReportMessage::ButtonReportCountByYearTable => {
                let mut d = vec![];
                if let Some(user) = &self.user_select_for_date_selected {
                    d = self
                        .count_by_year_user
                        .iter()
                        .filter(|(_, u, _)| u == user)
                        .map(|(y, _, c)| (*y, *c))
                        .collect()
                };
                let d = if self.user_select_for_date_selected.is_some() {
                    d.iter()
                } else {
                    self.count_by_year.iter()
                }
                .map(|(year, count)| (format!("{}年", year), *count))
                .collect::<Vec<_>>();
                self.report_image = Some(Self::rgb_to_rgba(&Self::get_report_histogram_image(
                    "以年份为维度统计总数",
                    "年份",
                    "总数",
                    |index| match index {
                        SegmentValue::Exact(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::CenterOf(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::Last => "".to_string(),
                    },
                    &d,
                    theme,
                )));
            }
            AnalysisDatabaseReportMessage::SelectionListUserListForDate(t) => {
                self.user_select_for_date_selected = Some(t);
            }
            AnalysisDatabaseReportMessage::ButtonReportClearUserSelectedForDate => {
                self.user_select_for_date_selected = None;
            }
            AnalysisDatabaseReportMessage::ButtonReportJieBaTextRankJoined => {
                let jieba = Jieba::new();
                let s = self.get_msg_str_contents_for_jieba(msg_info, contact);
                let s = s.join("\n");
                let config = KeywordExtractConfig::builder()
                    .use_hmm(true)
                    .build()
                    .unwrap();
                let keyword_extractor = TextRank::new(5, config);
                let top = keyword_extractor.extract_keywords(&jieba, &s, 35, Self::get_allow_pos());
                let d = top
                    .iter()
                    .map(|key| (&key.keyword, key.weight as usize))
                    .collect::<Vec<_>>();
                self.report_image = Some(Self::rgb_to_rgba(&Self::get_report_histogram_image(
                    "词频分析",
                    "关键词",
                    "权重",
                    |index| match index {
                        SegmentValue::Exact(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::CenterOf(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::Last => "".to_string(),
                    },
                    &d,
                    theme,
                )));
            }
            AnalysisDatabaseReportMessage::ButtonReportClearUserSelectedForJieba => {
                self.user_select_for_jieba_selected = None;
            }
            AnalysisDatabaseReportMessage::SelectionListUserListForJieba(user) => {
                self.user_select_for_jieba_selected = Some(user);
            }
            AnalysisDatabaseReportMessage::ButtonReportJieJustCut => {
                let jieba = Jieba::new();
                let s = self.get_msg_str_contents_for_jieba(msg_info, contact);
                let s = s.join("\n");
                let tags = jieba.tag(&s, true);
                let mut keys_map = HashMap::new();
                let allow_pos = Self::get_allow_pos();
                let allow_pos = allow_pos.iter().map(|s| s.as_str()).collect::<Vec<_>>();
                for tag in tags {
                    if allow_pos.contains(&tag.tag) {
                        if let Some(v) = keys_map.get_mut(tag.word) {
                            *v = *v + 1;
                        } else {
                            keys_map.insert(tag.word, 1usize);
                        }
                    }
                }
                let mut keys = keys_map.iter().collect::<Vec<_>>();
                keys.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
                let d = if keys.len() > 35 {
                    &keys[0..35]
                } else {
                    &keys[0..]
                }
                .iter()
                .map(|(key, weight)| (**key, **weight))
                .collect::<Vec<_>>();
                self.report_image = Some(Self::rgb_to_rgba(&Self::get_report_histogram_image(
                    "词频分析",
                    "关键词",
                    "总数",
                    |index| match index {
                        SegmentValue::Exact(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.to_string()
                            }
                        }
                        SegmentValue::CenterOf(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.to_string()
                            }
                        }
                        SegmentValue::Last => "".to_string(),
                    },
                    &d,
                    theme,
                )));
            }
            AnalysisDatabaseReportMessage::ButtonReportJieBaTfIdf => {
                let jieba = Jieba::new();
                let s = self.get_msg_str_contents_for_jieba(msg_info, contact);
                let s = s.join("\n");
                let keyword_extractor = TfIdf::default();
                let top = keyword_extractor.extract_keywords(&jieba, &s, 35, Self::get_allow_pos());
                let d = top
                    .iter()
                    .map(|key| (&key.keyword, (key.weight * 10000.0) as usize))
                    .collect::<Vec<_>>();
                self.report_image = Some(Self::rgb_to_rgba(&Self::get_report_histogram_image(
                    "词频分析",
                    "关键词",
                    "权重",
                    |index| match index {
                        SegmentValue::Exact(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::CenterOf(index) => {
                            if *index > d.len() {
                                "".to_string()
                            } else {
                                d[*index - 1].0.clone()
                            }
                        }
                        SegmentValue::Last => "".to_string(),
                    },
                    &d,
                    theme,
                )));
            }
        }
        iced::Task::none()
    }

    fn get_report_background_color(theme: &iced::theme::Theme) -> RGBColor {
        match theme {
            iced::theme::Theme::Light => plotters::style::WHITE,
            iced::theme::Theme::Dark => plotters::style::BLACK,
            _ => todo!()
        }
    }

    fn get_report_text_color(theme: &iced::theme::Theme) -> RGBColor {
        match theme {
            iced::theme::Theme::Light => plotters::style::BLACK,
            iced::theme::Theme::Dark => plotters::style::WHITE,
            _ => todo!()
        }
    }

    fn get_report_bold_line_color(theme: &iced::theme::Theme) -> RGBAColor {
        match theme {
            iced::theme::Theme::Light => plotters::style::WHITE.mix(0.3),
            iced::theme::Theme::Dark => RGBColor(200, 200, 200).mix(0.3),
            _ => todo!(),
        }
    }

    fn get_report_axis_color(theme: &iced::theme::Theme) -> RGBColor {
        match theme {
            iced::theme::Theme::Light => plotters::style::BLACK,
            iced::theme::Theme::Dark => plotters::style::WHITE,
            _ => todo!()
        }
    }

    fn get_report_line_series_image_order_by_date(
        title: &str,
        d: &[(NaiveDate, String, usize)],
        theme: &iced::theme::Theme,
    ) -> Vec<u8> {
        let mut vec = vec![0u8; REPORT_IMAGE_WIDTH as usize * REPORT_IMAGE_HEIGHT as usize * 3];
        {
            let root =
                BitMapBackend::with_buffer(vec.as_mut(), (REPORT_IMAGE_WIDTH, REPORT_IMAGE_HEIGHT))
                    .into_drawing_area();
            root.fill(&Self::get_report_background_color(theme)).unwrap();
            let mut chart = ChartBuilder::on(&root)
                .x_label_area_size(35)
                .y_label_area_size(40)
                .margin(5)
                .caption(title, ("楷体", 50.0, &Self::get_report_text_color(theme)))
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
                .bold_line_style(&Self::get_report_bold_line_color(theme))
                .axis_style(&Self::get_report_axis_color(theme))
                .y_desc("总数")
                .y_label_style(&Self::get_report_text_color(theme))
                .x_desc("排名")
                .y_label_style(&Self::get_report_text_color(theme))
                .axis_desc_style(("楷体", 15, &Self::get_report_text_color(theme)))
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
                    color: Self::get_report_text_color(theme).into(),
                    filled: false,
                    stroke_width: 0,
                })
                .label_font(("楷体", 20, &Self::get_report_text_color(theme)))
                .draw()
                .unwrap();
            root.present().unwrap();
        }
        vec
    }

    fn get_report_histogram_image<F>(
        title: &str,
        x_desc: &str,
        y_desc: &str,
        x_formatter: F,
        d: &[(impl ToString + Clone + Eq + Hash, usize)],
        theme: &iced::theme::Theme,
    ) -> Vec<u8>
    where
        F: Fn(&SegmentValue<usize>) -> String,
    {
        let mut vec = vec![0u8; REPORT_IMAGE_WIDTH as usize * REPORT_IMAGE_HEIGHT as usize * 3];
        {
            let root =
                BitMapBackend::with_buffer(vec.as_mut(), (REPORT_IMAGE_WIDTH, REPORT_IMAGE_HEIGHT))
                    .into_drawing_area();
            root.fill(&Self::get_report_background_color(theme)).unwrap();
            let mut chart = ChartBuilder::on(&root)
                .x_label_area_size(35)
                .y_label_area_size(40)
                .margin(5)
                .caption(title, ("楷体", 50.0, &Self::get_report_text_color(theme)))
                .build_cartesian_2d(
                    (1..d.len()).into_segmented(),
                    0..d.iter().map(|(_, count)| *count).max().unwrap_or(0),
                )
                .unwrap();
            chart
                .configure_mesh()
                .x_label_formatter(&x_formatter)
                .disable_x_mesh()
                .axis_style(&Self::get_report_axis_color(theme))
                .bold_line_style(&Self::get_report_bold_line_color(theme))
                .y_desc(y_desc)
                .y_label_style(&Self::get_report_text_color(theme))
                .x_desc(x_desc)
                .x_label_style(&Self::get_report_text_color(theme))
                .axis_desc_style(("楷体", 15, &Self::get_report_text_color(theme)))
                .draw()
                .unwrap();
            let mut color_map = HashMap::new();
            let mut d_order_by_count = d.to_vec();
            d_order_by_count.sort_by(|a, b| b.1.cmp(&a.1));
            for (index, (name, _)) in d_order_by_count.iter().enumerate() {
                color_map.insert(
                    name,
                    plotters::style::RED
                        .mix(0.3 + index as f64 * (0.5 / d.len() as f64))
                        .filled(),
                );
            }
            for (index, (name, count)) in d.iter().enumerate() {
                chart
                    .draw_series(
                        Histogram::vertical(&chart)
                            .style(*color_map.get(&name).unwrap())
                            .margin(20)
                            .data(vec![(index + 1, *count)]),
                    )
                    .unwrap()
                    .label(format!("{} {}", name.to_string(), count));
            }
            chart
                .configure_series_labels()
                .border_style(ShapeStyle {
                    color: Self::get_report_text_color(theme).into(),
                    filled: false,
                    stroke_width: 0,
                })
                .label_font(("楷体", 17, &Self::get_report_text_color(theme)))
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
    fn get_msg_str_contents_for_jieba<'a>(
        &self,
        msg_info: &'a MsgInfo,
        contact: &HashMap<Option<String>, Contact>,
    ) -> Vec<&'a str> {
        msg_info
            .msg_list
            .iter()
            .filter(|(_, msg)| msg.r#type == Some(1) && msg.str_content.is_some())
            .filter(|(_, msg)| {
                if let Some(user_for_select) = &self.user_select_for_jieba_selected {
                    let user = if msg.is_sender.unwrap() == 1 {
                        None
                    } else {
                        if let Some(data_extra) = &msg.bytes_extra {
                            if let Some(user_name) = data_extra.map.get(&1) {
                                Some(user_name[0].clone())
                            } else {
                                Some(msg.str_talker.as_ref().unwrap().clone())
                            }
                        } else {
                            Some(msg.str_talker.as_ref().unwrap().clone())
                        }
                    };
                    &self.get_user_name(&user, contact) == user_for_select
                } else {
                    true
                }
            })
            .map(|(_, msg)| msg.str_content.as_ref().unwrap().as_str())
            .collect::<Vec<_>>()
    }
    pub fn get_user_name(
        &self,
        user: &Option<String>,
        contact: &HashMap<Option<String>, Contact>,
    ) -> UserForSelect {
        let user = if let Some(user) = user {
            if let Some(contact) = contact.get(&Some(user.to_owned())) {
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
                UserForSelect::new(contact.user_name.clone(), text)
            } else {
                UserForSelect::new(Some("未知用户".to_string()), "未知用户".to_string())
            }
        } else {
            UserForSelect::new(Some("本人".to_string()), "本人".to_string())
        };
        user
    }
    fn get_allow_pos() -> Vec<String> {
        vec![
            String::from("n"),
            String::from("ns"),
            String::from("nr"),
            String::from("nt"),
            String::from("nz"),
            String::from("v"),
            String::from("vn"),
            String::from("i"),
            String::from("l"),
            String::from("e"),
            String::from("o"),
            String::from("r"),
            String::from("rr"),
        ]
    }
    pub fn draw(&self) -> Container<Message> {
        if let Some(err) = self.err_info.as_ref() {
            Container::new(
                Text::new(err).style(|_| iced::widget::text::Style{
                    color: Some(Color::from_rgb(1.0, 0.0, 0.0))
                }),
            )
        } else {
            Container::new({
                let mut col = Column::new();
                col = col.push(Text::new(format!("总数：{}", self.all_msg_count)));
                col =
                    col.push(
                        Row::new()
                            .push(Text::new("不同用户的聊天记录总数"))
                            .push(Text::new("前"))
                            .push(
                                TextInput::new("", &self.count_by_user_top_start)
                                    .on_input(|input| {
                                        AnalysisDatabaseReportMessage::InputReportCountByUserTopStart(
                                            input,
                                        ).into()
                                    }),
                            )
                            .push(Text::new("-"))
                            .push(
                                TextInput::new("", &self.count_by_user_top_end)
                                    .on_input(|input| {
                                        AnalysisDatabaseReportMessage::InputReportCountByUserTopEnd(
                                            input,
                                        ).into()
                                    }),
                            )
                            .push(Button::new("生成图表").on_press(
                                AnalysisDatabaseReportMessage::ButtonReportCountByUserTable.into(),
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
                                            self.count_by_date_start_show_picker,
                                            self.count_by_date_start,
                                            Button::new(Text::new(self.count_by_date_start.to_string())).on_press(AnalysisDatabaseReportMessage::DatePickerReportCountByDateStartUnderlay.into()),
                                            AnalysisDatabaseReportMessage::DatePickerReportCountByDateStartCancel.into(),
                                            |date| AnalysisDatabaseReportMessage::DatePickerReportCountByDateStartSubmit(date.into()).into(),
                                        )
                                    )
                                    .push(Text::new("-"))
                                    .push(
                                        DatePicker::new(
                                            self.count_by_date_end_show_picker,
                                            self.count_by_date_end,
                                            Button::new(Text::new(self.count_by_date_end.to_string())).on_press(AnalysisDatabaseReportMessage::DatePickerReportCountByDateEndUnderlay.into()),
                                            AnalysisDatabaseReportMessage::DatePickerReportCountByDateEndCancel.into(),
                                            |date| AnalysisDatabaseReportMessage::DatePickerReportCountByDateEndSubmit(date.into()).into(),
                                        )
                                    )
                                ).width(Length::Fill)
                            )
                            .push(Button::new("生成图表").on_press(
                                AnalysisDatabaseReportMessage::ButtonReportCountByDateOrderByDateTable.into(),
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
                                            self.count_by_date_top_start_show_picker,
                                            self.count_by_date_top_start_date,
                                            Button::new(Text::new(self.count_by_date_top_start_date.to_string())).on_press(AnalysisDatabaseReportMessage::DatePickerReportCountByDateTopStartUnderlay.into()),
                                            AnalysisDatabaseReportMessage::DatePickerReportCountByDateTopStartCancel.into(),
                                            |date| AnalysisDatabaseReportMessage::DatePickerReportCountByDateTopStartSubmit(date.into()).into(),
                                        )
                                    )
                                    .push(Text::new("-"))
                                    .push(
                                        DatePicker::new(
                                            self.count_by_date_top_end_show_picker,
                                            self.count_by_date_top_end_date,
                                            Button::new(Text::new(self.count_by_date_top_end_date.to_string())).on_press(AnalysisDatabaseReportMessage::DatePickerReportCountByDateTopEndUnderlay.into()),
                                            AnalysisDatabaseReportMessage::DatePickerReportCountByDateTopEndCancel.into(),
                                            |date| AnalysisDatabaseReportMessage::DatePickerReportCountByDateTopEndSubmit(date.into()).into()
                                        )
                                    )
                                )
                            ).push(Text::new("前"))
                            .push(
                                TextInput::new("", &self.count_by_date_top_start)
                                    .on_input(|input| {
                                        AnalysisDatabaseReportMessage::InputReportCountByDateTopStart(
                                            input,
                                        ).into()
                                    }),
                            )
                            .push(Text::new("-"))
                            .push(
                                TextInput::new("", &self.count_by_date_top_end)
                                    .on_input(|input| {
                                        AnalysisDatabaseReportMessage::InputReportCountByDateTopEnd(
                                            input,
                                        ).into()
                                    }),
                            )
                            .push(Button::new("生成图表").on_press(
                                AnalysisDatabaseReportMessage::ButtonReportCountByDateTable.into(),
                            )),
                    ).push(
                        Row::new().push(
                            Text::new("不同时间维度")
                        ).push(
                            PickList::new(self.user_select_for_date_options.clone(),self.user_select_for_date_selected.clone(), |t| AnalysisDatabaseReportMessage::SelectionListUserListForDate(t).into())
                        ).push(
                            Button::new("清除用户选择").on_press(AnalysisDatabaseReportMessage::ButtonReportClearUserSelectedForDate.into())
                        )
                        .push(
                            Button::new("年份").on_press(AnalysisDatabaseReportMessage::ButtonReportCountByYearTable.into())
                        ).push(
                            Button::new("月份").on_press(AnalysisDatabaseReportMessage::ButtonReportCountByMonthTable.into())
                        ).push(
                            Button::new("日期").on_press(AnalysisDatabaseReportMessage::ButtonReportCountByDayTable.into())
                        ).push(
                            Button::new("星期").on_press(AnalysisDatabaseReportMessage::ButtonReportCountByWeekdayTable.into())
                        ).push(
                            Button::new("小时").on_press(AnalysisDatabaseReportMessage::ButtonReportCountByHourTable.into())
                        )
                        .spacing(5)
                    ).push(
                        Row::new().push(
                            Text::new("词频分析")
                        ).push(
                            PickList::new(self.user_select_for_jieba_options.clone(),self.user_select_for_jieba_selected.clone(), |t| AnalysisDatabaseReportMessage::SelectionListUserListForJieba(t).into())
                        ).push(
                            Button::new("清除用户选择").on_press(AnalysisDatabaseReportMessage::ButtonReportClearUserSelectedForJieba.into())
                        ).push(
                            Button::new("TextRank").on_press(AnalysisDatabaseReportMessage::ButtonReportJieBaTextRankJoined.into())
                        ).push(
                            Button::new("TfIdf").on_press(AnalysisDatabaseReportMessage::ButtonReportJieBaTfIdf.into())
                        ).push(
                            Button::new("仅分词").on_press(AnalysisDatabaseReportMessage::ButtonReportJieJustCut.into())
                        )
                        .spacing(5)
                    );
                if let Some(image) = &self.report_image {
                    col = col.push(Image::new(Handle::from_rgba(
                        REPORT_IMAGE_WIDTH,
                        REPORT_IMAGE_HEIGHT,
                        image.clone(),
                    )));
                }
                col
            })
        }
    }
}

#[derive(Debug, Clone)]
pub enum AnalysisDatabaseReportMessage {
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
    ButtonReportClearUserSelectedForDate,
    ButtonReportClearUserSelectedForJieba,
    ButtonReportCountByYearTable,
    ButtonReportCountByMonthTable,
    ButtonReportCountByDayTable,
    ButtonReportCountByWeekdayTable,
    ButtonReportCountByHourTable,
    SelectionListUserListForDate(UserForSelect),
    SelectionListUserListForJieba(UserForSelect),
    ButtonReportJieBaTextRankJoined,
    ButtonReportJieBaTfIdf,
    ButtonReportJieJustCut,
}

impl Into<Message> for AnalysisDatabaseReportMessage {
    fn into(self) -> Message {
        Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::AnalysisDatabaseReportMessage(
            self,
        ))
    }
}
