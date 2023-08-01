use crate::app::Input;

pub fn move_cursor_left(input: &mut Input) {
    let new_pos = if input.cursor_position == 0 {
        0
    } else {
        input.cursor_position - 1
    };

    input.cursor_position = new_pos.clamp(0, input.text.chars().count().try_into().unwrap());
}

pub fn move_cursor_right(input: &mut Input) {
    let new_pos = input.cursor_position + 1;

    input.cursor_position = new_pos.clamp(0, input.text.chars().count().try_into().unwrap());
}

pub fn move_cursor_up(input: &mut Input) {
    unimplemented!()
}

pub fn move_cursor_down(input: &mut Input) {
    unimplemented!()
}

pub fn move_cursor_to_start_of_line(input: &mut Input) {
    unimplemented!()
}

pub fn move_cursor_to_end_of_line(input: &mut Input) {
    input.cursor_position = input.text.len().try_into().unwrap();
}

pub fn remove_char_before_cursor(input: &mut Input) {
    let removable = input.cursor_position != 0;

    if !removable {
        return;
    }

    input.text.remove(input.cursor_position as usize - 1);
    input.cursor_position -= 1;
}

pub fn add_char_at_cursor(input: &mut Input, c: char) {
    input.text.insert(input.cursor_position.into(), c);

    input.cursor_position += 1;
}
