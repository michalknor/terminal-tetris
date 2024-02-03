#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Colors {
	WHITE,
	GRAY,
	RED,
	GREEN,
	YELLOW,
	BLUE,
	PINK,
	CYAN
}


impl Colors {
    fn get_code_for_text(&self) -> i32 {
        match self {
            Colors::WHITE => 29,
			Colors::GRAY => 30,
			Colors::RED => 31,
			Colors::GREEN => 32,
			Colors::YELLOW => 33,
			Colors::BLUE => 34,
			Colors::PINK => 35,
			Colors::CYAN => 36,
        }
    }

    fn get_code_for_background(&self) -> i32 {
        match self {
            Colors::WHITE => 47,
			Colors::GRAY => 100,
			Colors::RED => 41,
			Colors::GREEN => 42,
			Colors::YELLOW => 43,
			Colors::BLUE => 44,
			Colors::PINK => 45,
			Colors::CYAN => 46,
        }
    }
}



pub fn colorize_text(text: String, text_color: Colors) -> String {
	format!("\x1b[{}m{}\x1b[0m", text_color.get_code_for_text(), text)
}


pub fn colorize_background(text: String, text_color: Colors) -> String {
	format!("\x1b[{}m{}\x1b[0m", text_color.get_code_for_background(), text)
}