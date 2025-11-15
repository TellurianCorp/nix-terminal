use iced::font::Font;
use iced::widget::{Column, Row, container, scrollable, text, text_input};
use iced::{Alignment, Application, Command, Element, Length, Theme, executor, theme};

/// Text that appears when the terminal first boots.
const STARTUP_LINES: &[&str] = &[
    "Tellurian nix-terminal",
    "----------------------",
    "Rendering UI mock for future terminal backend.",
];

#[derive(Default)]
pub struct NixTerminalApp {
    history: Vec<String>,
    current_input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    SubmitInput,
}

impl Application for NixTerminalApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        let history = STARTUP_LINES.iter().map(|line| line.to_string()).collect();
        (
            Self {
                history,
                current_input: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Tellurian nix-terminal")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::InputChanged(value) => self.current_input = value,
            Message::SubmitInput => {
                let trimmed = self.current_input.trim();
                if trimmed.is_empty() {
                    self.current_input.clear();
                    return Command::none();
                }

                let command = std::mem::take(&mut self.current_input);
                self.history.push(format!("$ {command}"));
                self.history
                    .push(String::from("# TODO: wire command execution backend"));
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let history_column = self
            .history
            .iter()
            .fold(Column::new().spacing(4), |column, line| {
                column.push(text(line).font(Font::MONOSPACE).size(16))
            });

        let output_panel = scrollable(history_column)
            .width(Length::Fill)
            .height(Length::Fill);

        let input_field = text_input("Type a command...", &self.current_input)
            .on_input(Message::InputChanged)
            .on_submit(Message::SubmitInput)
            .padding(12)
            .size(16)
            .font(Font::MONOSPACE);

        let input_row = Row::new()
            .spacing(8)
            .align_items(Alignment::Center)
            .push(text("$").font(Font::MONOSPACE))
            .push(input_field);

        let layout = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(0)
            .push(
                container(output_panel)
                    .padding(16)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(theme::Container::Box),
            )
            .push(
                container(input_row)
                    .width(Length::Fill)
                    .padding([8, 16, 16, 16])
                    .style(theme::Container::Transparent),
            );

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Container::Transparent)
            .into()
    }
}
