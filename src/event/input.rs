use crate::app::Coordinates;

#[derive(Clone)]
pub struct Input {
    pub text: String,
    pub cursor_position: Coordinates,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            text: String::from(""),
            cursor_position: Coordinates::default(),
        }
    }
}

impl Input {
    pub fn move_cursor_left(&mut self) {
        let new_pos = if self.cursor_position.x == 0 {
            0
        } else {
            self.cursor_position.x - 1
        };

        self.cursor_position.x = new_pos.clamp(0, self.text.chars().count().try_into().unwrap());
    }

    pub fn move_cursor_right(&mut self) {
        let (_, text) = self
            .text
            .split("\n")
            .enumerate()
            .find(|(index, _)| *index == usize::from(self.cursor_position.y))
            .unwrap_or((0, self.text.as_str()));

        let new_pos = self.cursor_position.x + 1;

        self.cursor_position.x = new_pos.clamp(0, text.chars().count().try_into().unwrap());
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor_position.y -= if self.cursor_position.y == 0 { 0 } else { 1 };
        self.cursor_position.x = 0;
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor_position.x = 0;

        let new_pos_y = self.cursor_position.y + 1;
        let lines: u16 = self.text.split("\n").count().try_into().unwrap();

        self.cursor_position.y = new_pos_y.clamp(0, if lines == 0 { 0 } else { lines - 1 })
    }

    pub fn move_cursor_to_start_of_line(&mut self) {
        self.cursor_position.x = 0;
    }

    pub fn move_cursor_to_end_single_line(&mut self) {
        self.cursor_position.x = self.text.chars().count().try_into().unwrap();
    }

    pub fn move_cursor_to_end_multi_line(&mut self) {
        let (_, text) = self
            .text
            .split("\n")
            .enumerate()
            .find(|(index, _)| *index == usize::from(self.cursor_position.y))
            .unwrap_or((0, self.text.as_str()));

        let lines: u16 = self.text.split("\n").count().try_into().unwrap();

        let text_len: u16 = text.chars().count().try_into().unwrap();

        self.cursor_position.x = text_len + if lines == 0 { 0 } else { 1 };
    }

    pub fn remove_char_before_cursor(&mut self) {
        if self.text.split("\n").count() <= 1 {
            self.remove_char_before_cursor_single_line();
        } else {
            self.remove_char_before_cursor_multi_line();
        }
    }

    pub fn remove_char_before_cursor_single_line(&mut self) {
        let removable = self.cursor_position.x != 0;

        if !removable {
            return;
        }

        self.text.remove(self.cursor_position.x as usize - 1);

        self.move_cursor_left();
    }

    fn remove_char_before_cursor_multi_line(&mut self) {
        let mut removed_line: bool = false;

        let new_text = self
            .text
            .split("\n")
            .enumerate()
            .map(|(index, line)| {
                let is_cursor_in_line = index == usize::from(self.cursor_position.y);

                let is_cursor_in_next_line = index + 1 == usize::from(self.cursor_position.y);

                if is_cursor_in_next_line && self.cursor_position.x == 0 {
                    removed_line = true;
                    return String::from(line);
                }

                if !is_cursor_in_line || (self.cursor_position.x == 0 && index == 0) {
                    return format!("{line}\n");
                }

                if self.cursor_position.x == 0 {
                    return format!("{line}\n");
                }

                let mut new_line = String::from(line);

                new_line.remove(self.cursor_position.x as usize - 1);
                removed_line = false;
                format!("{new_line}\n")
            })
            .collect::<Vec<String>>()
            .join("");

        self.text = new_text;

        if removed_line {
            self.move_cursor_up();
            self.move_cursor_to_end_multi_line();
        }

        self.move_cursor_left();
    }

    pub fn add_char_at_cursor(&mut self, c: char) {
        if self.text.split("\n").count() <= 1 {
            self.add_char_at_cursor_single_line(c);
        } else {
            self.add_char_at_cursor_multi_line(c);
        }
    }

    fn add_char_at_cursor_single_line(&mut self, c: char) {
        self.text.insert(self.cursor_position.x.into(), c);

        self.move_cursor_right()
    }

    fn add_char_at_cursor_multi_line(&mut self, c: char) {
        let new_text = self
            .text
            .split("\n")
            .enumerate()
            .map(|(index, line)| {
                let is_cursor_in_line = index == usize::from(self.cursor_position.y);

                if !is_cursor_in_line {
                    return String::from(line);
                }

                let mut new_line = String::from(line);

                new_line.insert(self.cursor_position.x.into(), c);

                new_line
            })
            .collect::<Vec<String>>()
            .join("\n");

        self.text = new_text;
        self.move_cursor_right()
    }

    pub fn add_newline_at_cursor(&mut self) {
        self.add_char_at_cursor('\n');

        self.move_cursor_to_start_of_line();
        self.move_cursor_down();
    }
}
