pub mod message;

use iced::{Element, Task, widget::text};
use message::Message;

#[derive(Default)]
pub struct OxideApp {
    // placeholder for future state, e.g., text buffer, settings
}

impl OxideApp {
    pub fn view(&self) -> Element<'_, Message> {
        text("Hello, world!").into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        println!("{:?}", message); // just to make rust shutup about it being unused
        Task::none()
    }
}
