use crate::app::{App, AppBlock};

pub fn scroll_down_response(app: &mut App) {
    if let AppBlock::Response = app.selected_block {
        let x = app.response_scroll.0 + 2;

        app.response_scroll.0 = x;
    }
}

pub fn scroll_up_response(app: &mut App) {
    if let AppBlock::Response = app.selected_block {
        let x = if app.response_scroll.0 == 0 {
            0
        } else {
            if app.response_scroll.0 > 2 {
                app.response_scroll.0 - 2
            } else {
                0
            }
        };

        app.response_scroll.0 = x;
    }
}
