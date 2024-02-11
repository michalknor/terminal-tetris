#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Color {
	WHITE,
	GRAY,
	RED,
	GREEN,
	YELLOW,
	BLUE,
	PINK,
	CYAN,
	NONE,
}


impl Color {
    fn get_code_for_text(&self) -> i32 {
        match self {
            Color::WHITE => 29,
			Color::GRAY => 30,
			Color::RED => 31,
			Color::GREEN => 32,
			Color::YELLOW => 33,
			Color::BLUE => 34,
			Color::PINK => 35,
			Color::CYAN => 36,
			Color::NONE => 0,
        }
    }

    fn get_code_for_background(&self) -> i32 {
        match self {
            Color::WHITE => 47,
			Color::GRAY => 100,
			Color::RED => 41,
			Color::GREEN => 42,
			Color::YELLOW => 43,
			Color::BLUE => 44,
			Color::PINK => 45,
			Color::CYAN => 46,
			Color::NONE => 0,
        }
    }
}



pub fn colorize_text(text: &String, text_color: Color) -> String {
	format!("\x1b[{}m{}\x1b[0m", text_color.get_code_for_text(), text)
}


pub fn colorize_background(text: &String, text_color: Color) -> String {
	format!("\x1b[{}m{}\x1b[0m", text_color.get_code_for_background(), text)
}