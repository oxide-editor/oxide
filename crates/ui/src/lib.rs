/*
 * Oxide - A opinionated code editor.
 * Copyright (C) 2026 Oxide Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use iced::widget::text;
use iced::{Element, Task};

// run the ui application
pub fn run() -> iced::Result {
    iced::application(
        || (OxideApp::default(), Task::none()),
        OxideApp::update,
        OxideApp::view,
    )
    .title(|_: &OxideApp| String::from("Oxide"))
    .run()
}

#[derive(Default)]
struct OxideApp;

#[derive(Debug, Clone)]
enum Message {}

impl OxideApp {
    // update app state
    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none() // no operation for now
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello, world!").into()
    }
}
