use crate::app::{App, AppBlock};

pub fn move_to_next_block(app: &mut App) {
    let mut selected_block: u16 = app.selected_block.clone().into();

    selected_block += 1;

    if selected_block > 5 {
        selected_block = 1;
    }

    app.selected_block = selected_block.into();
}

pub fn move_to_previous_block(app: &mut App) {
    let mut selected_block: u16 = app.selected_block.clone().into();

    selected_block -= 1;

    if selected_block == 0 {
        selected_block = 5;
    }

    app.selected_block = selected_block.into();
}

pub fn move_next_request_tab(app: &mut App) {
    let mut selected_tab: usize = app.request_tab.clone().into();

    selected_tab += 1;

    if selected_tab > 4 {
        selected_tab = 0;
    }

    app.request_tab = selected_tab.into();
}

pub fn move_to_previous_request_tab(app: &mut App) {
    let mut seleced_tab: usize = app.request_tab.clone().into();

    if seleced_tab == 0 {
        seleced_tab = 4;
    } else {
        seleced_tab -= 1;
    }

    app.request_tab = seleced_tab.into();
}

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
