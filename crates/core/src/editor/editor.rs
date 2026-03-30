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

use crate::input::Command;

#[derive(Debug, Default)]
pub struct Editor {
    pub buffer: String,
    pub cursor: usize,
}

impl Editor {
    pub fn apply_command(&mut self, command: Command) {
        match command {
            Command::Insert(c) => self.insert_char(c),
            Command::Backspace => self.delete_char(),
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert(self.cursor, c);
        self.cursor += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.buffer.remove(self.cursor);
        }
    }
}
