use crate::app::Input;

pub fn move_cursor_left(input: &mut Input) {
    let new_pos = if input.cursor_position.x == 0 {
        0
    } else {
        input.cursor_position.x - 1
    };

    input.cursor_position.x = new_pos.clamp(0, input.text.chars().count().try_into().unwrap());
}

pub fn move_cursor_right(input: &mut Input) {
    let (_, text) = input
        .text
        .split("\n")
        .enumerate()
        .find(|(index, _)| *index == usize::from(input.cursor_position.y))
        .unwrap_or((0, input.text.as_str()));

    let new_pos = input.cursor_position.x + 1;

    input.cursor_position.x = new_pos.clamp(0, text.chars().count().try_into().unwrap());
}

pub fn move_cursor_up(input: &mut Input) {
    input.cursor_position.y -= if input.cursor_position.y == 0 { 0 } else { 1 };
    input.cursor_position.x = 0;
}

pub fn move_cursor_down(input: &mut Input) {
    input.cursor_position.x = 0;

    let new_pos_y = input.cursor_position.y + 1;
    let lines: u16 = input.text.split("\n").count().try_into().unwrap();

    input.cursor_position.y = new_pos_y.clamp(0, if lines == 0 { 0 } else { lines - 1 })
}

pub fn move_cursor_to_start_of_line(input: &mut Input) {
    input.cursor_position.x = 0;
}

pub fn move_cursor_to_end_single_line(input: &mut Input) {
    input.cursor_position.x = input.text.chars().count().try_into().unwrap();
}

pub fn move_cursor_to_end_multi_line(input: &mut Input) {
    let (_, text) = input
        .text
        .split("\n")
        .enumerate()
        .find(|(index, _)| *index == usize::from(input.cursor_position.y))
        .unwrap_or((0, input.text.as_str()));

    let lines: u16 = input.text.split("\n").count().try_into().unwrap();

    let text_len: u16 = text.chars().count().try_into().unwrap();

    input.cursor_position.x = text_len + if lines == 0 { 0 } else { 1 };
}

pub fn remove_char_before_cursor(input: &mut Input) {
    if input.text.split("\n").count() <= 1 {
        remove_char_before_cursor_single_line(input);
    } else {
        remove_char_before_cursor_multi_line(input);
    }
}

pub fn remove_char_before_cursor_single_line(input: &mut Input) {
    let removable = input.cursor_position.x != 0;

    if !removable {
        return;
    }

    input.text.remove(input.cursor_position.x as usize - 1);

    move_cursor_left(input)
}

fn remove_char_before_cursor_multi_line(input: &mut Input) {
    let mut removed_line: bool = false;

    let new_text = input
        .text
        .split("\n")
        .enumerate()
        .map(|(index, line)| {
            let is_cursor_in_line = index == usize::from(input.cursor_position.y);

            let is_cursor_in_next_line = index + 1 == usize::from(input.cursor_position.y);

            if is_cursor_in_next_line && input.cursor_position.x == 0 {
                removed_line = true;
                return String::from(line);
            }

            if !is_cursor_in_line || (input.cursor_position.x == 0 && index == 0) {
                return format!("{line}\n");
            }

            if input.cursor_position.x == 0 {
                return format!("{line}\n");
            }

            let mut new_line = String::from(line);

            new_line.remove(input.cursor_position.x as usize - 1);
            removed_line = false;
            format!("{new_line}\n")
        })
        .collect::<Vec<String>>()
        .join("");

    input.text = new_text;

    if removed_line {
        move_cursor_up(input);
        move_cursor_to_end_multi_line(input);
    }

    move_cursor_left(input)
}

pub fn add_char_at_cursor(input: &mut Input, c: char) {
    if input.text.split("\n").count() <= 1 {
        add_char_at_cursor_single_line(input, c);
    } else {
        add_char_at_cursor_multi_line(input, c);
    }
}

fn add_char_at_cursor_single_line(input: &mut Input, c: char) {
    input.text.insert(input.cursor_position.x.into(), c);

    move_cursor_right(input)
}

fn add_char_at_cursor_multi_line(input: &mut Input, c: char) {
    let new_text = input
        .text
        .split("\n")
        .enumerate()
        .map(|(index, line)| {
            let is_cursor_in_line = index == usize::from(input.cursor_position.y);

            if !is_cursor_in_line {
                return String::from(line);
            }

            let mut new_line = String::from(line);

            new_line.insert(input.cursor_position.x.into(), c);

            new_line
        })
        .collect::<Vec<String>>()
        .join("\n");

    input.text = new_text;
    move_cursor_right(input)
}

pub fn add_newline_at_cursor(input: &mut Input) {
    add_char_at_cursor(input, '\n');

    move_cursor_to_start_of_line(input);
    move_cursor_down(input);
}
