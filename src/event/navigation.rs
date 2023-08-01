use crate::app::{App, AppBlock};

pub fn move_to_next_block(app: &mut App) {
    let mut selected_block: u16 = app.selected_block.clone().into();

    selected_block += 1;

    if selected_block > 4 {
        selected_block = 1;
    }

    app.selected_block = selected_block.into();
}

pub fn move_to_previous_block(app: &mut App) {
    let mut selected_block: u16 = app.selected_block.clone().into();

    selected_block -= 1;

    if selected_block == 0 {
        selected_block = 4;
    }

    app.selected_block = selected_block.into();
}

pub fn scroll_up_response(app: &mut App) {
    if let AppBlock::Response = app.selected_block {
        let x = app.response_scroll.0 + 2;

        app.response_scroll.0 = x;
    }
}

pub fn scroll_down_response(app: &mut App) {
    if let AppBlock::Response = app.selected_block {
        let x = if app.response_scroll.0 == 0 {
            0
        } else {
            if app.response_scroll.0 - 2 > 0 {
                app.response_scroll.0 - 2
            } else {
                0
            }
        };

        app.response_scroll.0 = x;
    }
}
