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

pub mod app;

use std::io::Cursor;

use app::OxideApp;
use iced::{Task, window::{self, Icon}};
use image::ImageReader;

pub fn run() -> iced::Result {
    iced::application(
        || (OxideApp::default(), Task::none()),
        OxideApp::update,
        OxideApp::view,
    )
    .title(|_: &OxideApp| "Oxide".to_string())
    .window(window::Settings {
        min_size: Some(iced::Size::new(600.0, 700.0)),
        icon: load_icon(),
        ..window::Settings::default()
    })
    .run()
}

fn load_icon() -> Option<Icon> {
    let png_bytes = include_bytes!("../../../assets/icon.png");

    let img = ImageReader::new(Cursor::new(png_bytes))
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()?
        .into_rgba8();
    
    let (width, height) = img.dimensions();
    window::icon::from_rgba(img.into_raw(), width, height).ok()
}