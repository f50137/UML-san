use std::io;
use std::fs;
use std::cmp;

mod java;
use java::*;
mod parser;
use parser::Parser;

use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;
const FONT_SIZE: i32 = 40;
const UML_PADDING: i32 = 20;
const UML_ITEM_HEIGHT: i32 = UML_PADDING * 2 + FONT_SIZE;

const SPACING: f32 = 3.0;

fn render_class_font(rl: &mut RaylibDrawHandle, class: &Class, start_x: i32, start_y: i32, font: &Font) {
    let field_strs = class.fields.iter().map(|f| f.to_string()).collect::<Vec<_>>();
    let method_strs = class.methods.iter().map(|f| f.to_string()).collect::<Vec<_>>();

    let max_field_text_width = field_strs.iter().map(|f| measure_text_ex(font, f, FONT_SIZE as f32, SPACING).x as i32).max().unwrap();
    let max_method_text_width = method_strs.iter().map(|f| measure_text_ex(font, f, FONT_SIZE as f32, SPACING).x as i32).max().unwrap();
    let class_name_width = measure_text_ex(font, &class.name, FONT_SIZE as f32, SPACING).x as i32;

    let uml_width = cmp::max(class_name_width, cmp::max(max_field_text_width, max_method_text_width)) + UML_PADDING * 2;
    let uml_height = (field_strs.len() + method_strs.len() + 1) as i32 * UML_ITEM_HEIGHT;

    rl.draw_rectangle_lines(start_x, start_y, uml_width, uml_height, Color::BLACK);
    rl.draw_text_ex(font, &class.name,
                    Vector2::new(
                        (start_x + uml_width / 2 - class_name_width / 2) as f32,
                        (start_y + UML_PADDING) as f32),
                    FONT_SIZE as f32, SPACING,
                    Color::BLACK);
    rl.draw_line(start_x, start_y + UML_ITEM_HEIGHT, start_x + uml_width, start_y + UML_ITEM_HEIGHT, Color::BLACK);

    for (i, text) in field_strs.iter().enumerate() {
        rl.draw_text_ex(font, text,
                        Vector2::new(
                            (start_x + UML_PADDING) as f32,
                            (start_y + (i as i32 + 1) * UML_ITEM_HEIGHT + UML_PADDING) as f32),
                        FONT_SIZE as f32, SPACING,
                        Color::BLACK);
    }
    
    let method_start_y = start_y + UML_ITEM_HEIGHT * (field_strs.len() as i32 + 1);
    rl.draw_line(start_x, method_start_y, start_x + uml_width, method_start_y, Color::BLACK);

    for (i, text) in method_strs.iter().enumerate() {
        rl.draw_text_ex(font, text,
                        Vector2::new(
                            (start_x + UML_PADDING) as f32,
                            (method_start_y + (i as i32) * UML_ITEM_HEIGHT + UML_PADDING) as f32),
                        FONT_SIZE as f32, SPACING,
                        Color::BLACK);
    }
}
fn main() -> io::Result<()> {
    let source = fs::read_to_string("sample.java")?;
    let mut parser = Parser::new(&source);
    let class = parser.parse_class_def();

    if parser.diagnostics.len() > 0 {
        for d in parser.diagnostics {
            println!("(pos {}) Parsing Error: {}", d.pos, d.message);
        }

        std::process::exit(1);
    }

    set_trace_log(TraceLogType::LOG_NONE);
    let (mut rl, thread) = raylib::init().size(WINDOW_WIDTH, WINDOW_HEIGHT).title("UML-san uwu").build();
    rl.set_target_fps(60);

    rl.set_window_state(rl.get_window_state().set_window_resizable(true));
    
    let font_result = rl.load_font(&thread, "font.ttf");
    let font;
    if font_result.is_err() {
        std::process::exit(1);
    } else {
        font = font_result.unwrap();
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        render_class_font(&mut d, &class, 23, 23, &font);
        //render_class(&mut d, &class, 23, 23);
    }

    Ok(())
}
