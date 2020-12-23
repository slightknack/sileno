use iced::{button, Sandbox, Element, Row, Column, Align, Button, Text, Settings, Length, Container};

pub fn run() {
    Counter::run(Settings::default());
}

#[derive(Default)]
struct Counter {
    value:     f64,
    increment: button::State,
    decrement: button::State,
    double:    button::State,
    half:      button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
    Double,
    Half,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        format!("Numbah Manipulatah: {}", self.value)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => { self.value += 1.0 }
            Message::Decrement => { self.value -= 1.0 }
            Message::Double    => { self.value *= 2.0 }
            Message::Half      => { self.value /= 2.0 }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let increase_value = Row::new()
            .spacing(5)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment, Text::new("Increment"))
                    .on_press(Message::Increment)
            )
            .push(
                Button::new(&mut self.double, Text::new("Double"))
                    .on_press(Message::Double)
            );

        let decrease_value = Row::new()
            .spacing(5)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.decrement, Text::new("Decrement"))
                    .on_press(Message::Decrement)
            )
            .push(
                Button::new(&mut self.half, Text::new("Half"))
                    .on_press(Message::Half)
            );

        let column = Column::new()
            .spacing(5)
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(increase_value)
            .push(Text::new(self.value.to_string()).size(128))
            .push(decrease_value);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_y()
            .into()
    }
}
