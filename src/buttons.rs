use macroquad::prelude::*;

fn x_percent(percent: f32) -> f32 {
    screen_width() / 100. * percent
}

fn y_percent(percent: f32) -> f32 {
    screen_height() / 100. * percent
}

pub fn play_button_area() -> Rect {
    let x_pos = x_percent(20.);
    let y_pos = y_percent(20.);
    let width = x_percent(20.);
    let height = y_percent(10.);

    return Rect::new(x_pos, y_pos, width, height);
}

pub fn settings_button_area() -> Rect {
    let x_pos = x_percent(20.);
    let y_pos = y_percent(40.);
    let width = x_percent(20.);
    let height = y_percent(10.);

    return Rect::new(x_pos, y_pos, width, height);
}

pub fn quit_button_area() -> Rect {
    let x_pos = x_percent(20.);
    let y_pos = y_percent(60.);
    let width = x_percent(20.);
    let height = y_percent(10.);

    return Rect::new(x_pos, y_pos, width, height);
}

pub fn draw_button_rectangle(label: &str, rect: Rect, rect_color: Color) {
    let font_size = 60.;
    let text_color = BLACK;
    let x = rect.x + (rect.w * 0.10);
    let y = rect.y + (rect.h * 0.66);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, rect_color);
    draw_text(label, x, y, font_size, text_color);
}
