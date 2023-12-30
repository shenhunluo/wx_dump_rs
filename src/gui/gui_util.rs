use std::sync::RwLock;
use cpal::{traits::{HostTrait, DeviceTrait, StreamTrait}, SampleFormat, StreamConfig, Stream};
use iced::{widget::{Column, Text, TextInput, Row, Container, Button, Space, Scrollable}, alignment::{Vertical, Horizontal}, Color, Length};
use iced_runtime::Command;

use super::Message;

enum SetColEnum {
    Text,
    TextInput,
}

fn set_col<'a,F,M,ID, BD>(col: Column<'a,Message>,text: &'a str, input_msg: F, btm_msg: Option<M>, input_data: ID, btm_data: BD, err_msg: &Option<impl ToString> ,get_center_ele: SetColEnum) -> Column<'a,Message>
where 
    F: 'a + Fn(String) -> M,
    M: Into<Message> + Clone,
    ID: ToString,
    BD: ToString,
{
    let mut col = col.push(
        Row::new()
                        .spacing(5)
                        .push(
                            Text::new(text)
                                .vertical_alignment(Vertical::Center)
                                .width(240)
                                .horizontal_alignment(Horizontal::Left)
                                .height(30)
                            )
                        .push(
                            match get_center_ele {
                                SetColEnum::Text => {
                                    Container::new(
                                        Text::new(input_data.to_string())
                                    )
                                },
                                SetColEnum::TextInput => {
                                    Container::new(
                                        TextInput::new(&format!("请输入{}",text.to_string()), &input_data.to_string())
                                            .on_input(move |e| Into::<Message>::into(input_msg(e)))
                                    )
                                },
                            }.width(Length::Fill)
                        ).push(
                            if let Some(btm_msg) = btm_msg {
                                Container::new(
                                    Button::new(Text::new(btm_data.to_string()).horizontal_alignment(Horizontal::Center))
                                        .on_press(Into::<Message>::into(btm_msg)).height(30).width(50)
                                )
                            } else {
                                Container::new(Space::with_width(50))
                            }
                        )
    );
    if let Some(msg) = err_msg {
        col = col.push(
            Text::new(msg.to_string()).style(iced::theme::Text::Color(Color::from_rgb(1.0, 0.0, 0.0)))
        )
    }
    col
}

pub fn set_col_with_text_input<'a,F,M,ID, BD>(col: Column<'a,Message>,text: &'a str, input_msg: F, btm_msg: Option<M>, input_data: ID, btm_data: BD, err_msg: &Option<impl ToString> ) -> Column<'a,Message>
where 
    F: 'a + Fn(String) -> M,
    M: Into<Message> + Clone,
    ID: ToString,
    BD: ToString,
{
    set_col(col, text, input_msg, btm_msg, input_data, btm_data, err_msg, SetColEnum::TextInput)
}

pub fn set_col_with_text<'a, M,ID, BD>(col: Column<'a,Message>,text: &'a str, btm_msg: Option<M>, input_data: ID, btm_data: BD, err_msg: &Option<impl ToString> ) -> Column<'a,Message>
where 
    M: Into<Message> + Clone,
    ID: ToString,
    BD: ToString,
{
    set_col(col, text, |_| panic!(), btm_msg, input_data, btm_data, err_msg, SetColEnum::Text)
}


pub struct PrintInfoText {
    data: RwLock<Vec<(String,Color)>>,
    last_len: RwLock<usize>,
    scroll_id: iced::widget::scrollable::Id,
}

impl PrintInfoText {
    pub fn new(id: iced::widget::scrollable::Id) -> Self {
        Self {
            data: RwLock::new(Vec::new()),
            last_len: RwLock::new(0),
            scroll_id: id,
        }
    }
    pub fn push_new_len(&self, str: impl ToString) {
        self.data.write().unwrap().push((str.to_string(),Color::BLACK));
    }
    pub fn push_new_err_len(&self, str: impl ToString) {
        self.data.write().unwrap().push((str.to_string(),Color::from_rgb(1.0, 0.0, 0.0)));
    }
    pub fn push_to_last(&self, str: impl ToString) {
        let mut data = self.data.write().unwrap();
        let len = data.len();
        data.get_mut(len-1).unwrap().0.push_str(&str.to_string());
    }
    pub fn check_scroll(&self) -> Command<Message> {
        let mut last_len = self.last_len.write().unwrap();
        let data_last_len = self.data.read().unwrap().len();
        if *last_len != data_last_len {
            *last_len = data_last_len;
            iced::widget::scrollable::snap_to::<Message>(self.scroll_id.clone(), iced::widget::scrollable::RelativeOffset{ x: 0.0, y: 1.0 })
        } else {
            Command::none()
        }
    }
    pub fn draw(&self) -> Container<Message> {
        Container::new(
            Scrollable::new(
                {
                    let mut col = Column::new().width(Length::Fill);
                    for (s,c) in self.data.read().unwrap().iter() {
                        col = col.push(Text::new(s.replace('\t',"    ").to_owned()).style(iced::theme::Text::Color(c.clone())))
                    }
                    col
                }
            )
            .id(self.scroll_id.clone()).height(Length::Fill)
        ).padding(4)
    }
}


pub fn play_audio<F>(get_data: F) -> Result<Stream,anyhow::Error> where F:FnOnce(u32)->Result<Vec<i16>,anyhow::Error> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or(anyhow::anyhow!("未找到音频设备"))?;
    let mut configs = device.supported_output_configs()?;
    let supported_config = configs.next().ok_or(anyhow::anyhow!("未找到配置"))?.with_max_sample_rate();
    let sample_format = supported_config.sample_format();
    let config: StreamConfig = supported_config.into();
    let output_data = get_data(config.sample_rate.0)?;
    let mut index = 0;
    let channels = config.channels as usize;
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, move |data, _| {
            for d in data.chunks_mut(channels) {
                for d in d.iter_mut() {
                    if index < output_data.len() {
                        *d = output_data[index] as f32 / i16::MAX as f32;
                    } else {
                        *d = 0.0;
                    }
                }
                if index < output_data.len() {
                    index += 1;
                }
            }
        }, err_fn, None),
        SampleFormat::I16 => device.build_output_stream(&config, move |data, _| {
            for d in data.chunks_mut(channels) {
                for d in d.iter_mut() {
                    if index < output_data.len() {
                        *d = output_data[index];
                    } else {
                        *d = 0i16;
                    }
                }
                if index < output_data.len() {
                    index += 1;
                }
            }
        }, err_fn, None),
        sample_format => panic!("Unsupported sample format '{sample_format}'")
    }.unwrap();
    stream.play()?;
    Ok(stream)
}