#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]
#[macro_use] // 必须添加此属性
extern crate lazy_static; // 显式声明宏导入:ml-citation{ref="1,8" data="citationList"}

mod serial;
use eframe::egui;
use eframe::epaint::text::{FontData, FontDefinitions, FontInsert, InsertFontFamily};
use eframe::epaint::FontFamily;
use egui::Align::Center;
use egui::{Margin, Widget};
use serial::Serial;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref SERIALS: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[tokio::main]
async fn main() -> eframe::Result {
    // let serial = Serial::new("COM6", 115200, 8, 1);
    // serial.read().await?;
    let icon_data = include_bytes!("../assets/32x32.png");
    let img = image::load_from_memory_with_format(icon_data, image::ImageFormat::Png).unwrap();
    let rgba_data = img.into_rgba8();
    let screen_size = get_primary_screen_size(); // 获取主屏幕尺寸
    let window_size = egui::vec2(900.0, 600.0); // 假设窗口尺寸
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(false)
            .with_min_inner_size(window_size)
            .with_position(egui::pos2(
                (screen_size.0 - window_size.x) / 2.0 + 1920.0,
                (screen_size.1 - window_size.y) / 2.0,
            ))
            .with_icon(egui::IconData {
                rgba: rgba_data.to_vec(),
                width: 32,
                height: 32,
            }),
        ..Default::default()
    };
    match tokio_serial::available_ports() {
        Ok(ports) => {
            for port in ports {
                SERIALS
                    .lock()
                    .unwrap()
                    .insert(port.port_name.clone(), port.port_name);
            }
        }
        Err(_) => {}
    }
    eframe::run_native(
        "byte watcher",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            setup_fonts(&cc.egui_ctx);
            Ok(Box::<ByteWatcherApp>::default())
        }),
    )
}

#[cfg(target_os = "windows")]
fn get_primary_screen_size() -> (f32, f32) {
    use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
    unsafe {
        (
            GetSystemMetrics(SM_CXSCREEN) as f32,
            GetSystemMetrics(SM_CYSCREEN) as f32,
        )
    }
}
fn setup_fonts(ctx: &egui::Context) {
    let font_data = include_bytes!("../assets/fonts/Source_Han_Sans_SC_Regular.otf");
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "Source_Han".to_owned(),
        Arc::from(FontData::from_static(font_data)),
    );
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .push("Source_Han".to_owned());
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .push("Source_Han".to_owned());
    ctx.set_fonts(fonts);
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ConnectType {
    SERIAL,
    TCP,
    UDP,
    WS,
}

pub struct ByteWatcherApp {
    connected: bool,
    connect_type: ConnectType,
    serial_connetct_info: SerialInfo,
    serial: Option<Serial>,
}
pub struct SerialInfo {
    path: String,
    baud_rate: u32,
    data_bits: u8,
    stop_bits: u8,
}
impl Default for ByteWatcherApp {
    fn default() -> Self {
        Self {
            connected: false,
            connect_type: ConnectType::SERIAL,
            serial_connetct_info: SerialInfo {
                path: "".into(),
                baud_rate: 115200,
                data_bits: 8,
                stop_bits: 1,
            },
            serial: None,
        }
    }
}
const LABLE_WIDTH: f32 = 200.0;
impl eframe::App for ByteWatcherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.override_font_id = Some(egui::FontId::new(16.0, FontFamily::Proportional));
        style.spacing.interact_size = egui::Vec2::new(0.0, 30.0); // 影响标签交互区域
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            // let left_width = ui.available_width() * 0.3;
            egui::SidePanel::left("left_panel")
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("通讯协议:")
                            .on_hover_cursor(egui::CursorIcon::Default);
                    });
                    ui.separator();
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.set_width(LABLE_WIDTH);
                        let connect_type = match self.connect_type {
                            ConnectType::SERIAL => "串口通讯",
                            ConnectType::TCP => "TCP client",
                            ConnectType::UDP => "UDP client",
                            ConnectType::WS => "WS client",
                        };
                        ui.with_layout(egui::Layout::left_to_right(Center), |ui| {
                            ui.label("通讯类型");
                        });
                        ui.with_layout(egui::Layout::right_to_left(Center), |ui| {
                            egui::ComboBox::from_id_salt("")
                                .selected_text(connect_type)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.connect_type,
                                        ConnectType::SERIAL,
                                        "串口通讯",
                                    );
                                    // ui.selectable_value(
                                    //     &mut self.connect_type,
                                    //     ConnectType::TCP,
                                    //     "TCP client",
                                    // );
                                    // ui.selectable_value(
                                    //     &mut self.connect_type,
                                    //     ConnectType::UDP,
                                    //     "UDP client",
                                    // );
                                    // ui.selectable_value(
                                    //     &mut self.connect_type,
                                    //     ConnectType::WS,
                                    //     "WS client",
                                    // );
                                });
                        });
                    });
                    match self.connect_type {
                        ConnectType::SERIAL => gen_serial_config_ui(ui, self),
                        ConnectType::TCP => gen_tcp_config_ui(ui, self),
                        ConnectType::UDP => gen_udp_config_ui(ui, self),
                        ConnectType::WS => gen_ws_config_ui(ui, self),
                    }
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.with_layout(
                            egui::Layout::centered_and_justified(egui::Direction::BottomUp),
                            |ui| {
                                let (btn_text, btn_color) = match self.connected {
                                    true => ("断开", egui::Color32::from_rgb(0xC2, 0x18, 0x5B)), // 断开状态显示红色
                                    false => ("连接", egui::Color32::from_rgb(0x19, 0x76, 0xD2)),
                                };
                                if ui
                                    .add_sized(
                                        [LABLE_WIDTH, 20.0],
                                        egui::Button::new(
                                            egui::RichText::new(btn_text)
                                                .color(egui::Color32::WHITE),
                                        )
                                        .fill(btn_color),
                                    )
                                    .clicked()
                                {
                                    if self.connected {
                                        self.connected = false;
                                        match self.serial.as_mut() {
                                            Some(serial) => {
                                                serial.close();
                                            }
                                            None => {}
                                        }
                                    } else {
                                        let serial = Serial::new(
                                            &self.serial_connetct_info.path,
                                            self.serial_connetct_info.baud_rate,
                                            self.serial_connetct_info.data_bits,
                                            self.serial_connetct_info.stop_bits,
                                        );
                                        self.serial = Some(serial);
                                        self.connected = true;
                                    }
                                }
                            },
                        );
                    });
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Copyright © 2025 saberzy")
                                .on_hover_cursor(egui::CursorIcon::Default);
                        });
                    });
                });
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("数据显示")
                        .on_hover_cursor(egui::CursorIcon::Default);
                });
                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 5.0;
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                        match self.serial.as_mut() {
                            Some(serial) => {
                                while let Ok(data) = serial.data_rx.try_recv() {
                                    // println!("aaaaa");
                                    // ui.code(format!("len:{}", data.len()));
                                    ui.code("asaaaaaa");
                                }
                            }
                            None => {}
                        }
                    });
            });
        });
    }
}
fn gen_serial_config_ui(ui: &mut egui::Ui, bw: &mut ByteWatcherApp) {
    ui.horizontal(|ui| {
        ui.set_width(LABLE_WIDTH);
        ui.with_layout(egui::Layout::left_to_right(Center), |ui| {
            ui.label("串口号");
        });
        ui.with_layout(egui::Layout::right_to_left(Center), |ui| {
            egui::ComboBox::from_id_salt("path")
                .selected_text(bw.serial_connetct_info.path.as_str())
                .width(100.0)
                .show_ui(ui, |ui| {
                    SERIALS.lock().unwrap().iter().for_each(|(k, v)| {
                        ui.selectable_value(&mut bw.serial_connetct_info.path, k.into(), v);
                    });
                });
        });
    });
    ui.horizontal(|ui| {
        ui.set_width(LABLE_WIDTH);
        ui.with_layout(egui::Layout::left_to_right(Center), |ui| {
            ui.label("波特率");
        });

        ui.with_layout(egui::Layout::right_to_left(Center), |ui| {
            ui.add_sized(
                [100.0, 20.0],
                egui::DragValue::new(&mut bw.serial_connetct_info.baud_rate).speed(0),
            )
            .on_hover_cursor(egui::CursorIcon::Text);
        });
    });
    ui.horizontal(|ui| {
        ui.set_width(LABLE_WIDTH);
        ui.with_layout(egui::Layout::left_to_right(Center), |ui| {
            ui.label("数据位");
        });
        ui.with_layout(egui::Layout::right_to_left(Center), |ui| {
            ui.add_sized(
                [100.0, 20.0],
                egui::DragValue::new(&mut bw.serial_connetct_info.data_bits).speed(0),
            )
            .on_hover_cursor(egui::CursorIcon::Text);
        });
    });
    ui.horizontal(|ui| {
        ui.set_width(LABLE_WIDTH);
        ui.with_layout(egui::Layout::left_to_right(Center), |ui| {
            ui.label("停止位");
        });
        ui.with_layout(egui::Layout::right_to_left(Center), |ui| {
            ui.add_sized(
                [100.0, 20.0],
                egui::DragValue::new(&mut bw.serial_connetct_info.stop_bits).speed(0),
            )
            .on_hover_cursor(egui::CursorIcon::Text);
        });
    });
}
fn gen_tcp_config_ui(ui: &mut egui::Ui, bw: &mut ByteWatcherApp) {
    ui.horizontal(|ui| {
        ui.label("通讯类型11")
            .on_hover_cursor(egui::CursorIcon::Default);
    });
}
fn gen_udp_config_ui(ui: &mut egui::Ui, bw: &mut ByteWatcherApp) {
    ui.horizontal(|ui| {
        ui.label("通讯类型12")
            .on_hover_cursor(egui::CursorIcon::Default);
    });
}
fn gen_ws_config_ui(ui: &mut egui::Ui, bw: &mut ByteWatcherApp) {
    ui.horizontal(|ui| {
        ui.label("通讯类型133")
            .on_hover_cursor(egui::CursorIcon::Default);
    });
}
#[test]
fn test_crc() {
    // let state: crc16::State<crc16::XMODEM> = crc16::State::new();
    let b = &[0x55, 0xaa, 0x1b, 0x00, 0x1f, 0x02, 0x00, 0x00];
    let res = crc16::State::<crc16::XMODEM>::calculate(b);
    println!("crc_{}", hex::encode(res.to_le_bytes()));

    let bb = &[
        0x16, 0x05, 0x00, 0x00, 0x08, 0x01, 0x10, 0xc5, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];
    let sum = bb.iter().fold(0u16, |acc, &x| acc + x as u16);
    println!("sum {}", hex::encode(sum.to_le_bytes()))
}
