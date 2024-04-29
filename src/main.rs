use clap::Command as ClapCommand;
use iced::widget::{column, container, slider, text, Container, Theme};
use iced::window::Settings;
use iced::{alignment::Horizontal, Alignment, Length, Size};
use std::process::{exit, Command};

#[derive(Clone, Debug)]
enum Message {
    BrightnessAdjust(u32),
}

struct App {
    current_brightness: u32,
    current_brightness_percentage: f32,
}

impl App {
    fn view(&self) -> Container<Message> {
        let control = slider(
            0..=get_max_brightness(),
            self.current_brightness,
            Message::BrightnessAdjust,
        )
        .step(1u32)
        .width(Length::Fill);
        let stat = text(format!("{:.2}%", self.current_brightness_percentage))
            .horizontal_alignment(Horizontal::Center);
        container(column![control, stat].align_items(Alignment::Center))
            .padding([2, 10])
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::BrightnessAdjust(new) => {
                self.current_brightness = new;
                self.current_brightness_percentage =
                    (new as f32 / get_max_brightness() as f32) * 100f32;
                set_brightness(new);
            }
        }
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinMacchiato
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_brightness: get_current_brightness(),
            current_brightness_percentage: (get_current_brightness() as f32
                / get_max_brightness() as f32)
                * 100f32,
        }
    }
}

fn main() {
    let app = ClapCommand::new("Brighten it")
        .subcommand(ClapCommand::new("run").about("Run the brightness control gui."))
        .subcommand(ClapCommand::new("info").about("Get the current brightness info."))
        .get_matches();
    match app.subcommand_name() {
        Some("run") => {
            let _ = run_app();
        }
        Some("info") => println!(
            "{:.2}%",
            get_current_brightness() as f32 / get_max_brightness() as f32 * 100f32
        ),
        _ => exit(0),
    }
}

fn run_app() -> iced::Result {
    iced::program("Brightness Control", App::update, App::view)
        .theme(App::theme)
        .settings(iced::Settings {
            window: Settings {
                size: Size::new(200., 50.),
                max_size: Some(Size::new(200., 50.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .run()
}

fn get_current_brightness() -> u32 {
    String::from_utf8(
        Command::new("brightnessctl")
            .arg("g")
            .output()
            .expect("Cannot run brightnessctl")
            .stdout,
    )
    .unwrap()
    .trim()
    .parse()
    .unwrap()
}

fn get_max_brightness() -> u32 {
    String::from_utf8(
        Command::new("brightnessctl")
            .arg("m")
            .output()
            .expect("Cannot run brightnessctl")
            .stdout,
    )
    .unwrap()
    .trim()
    .parse()
    .unwrap()
}

fn set_brightness(level: u32) {
    Command::new("brightnessctl")
        .arg("s")
        .arg(format!("{level}"))
        .output()
        .expect("Could not set brightness");
}
