#![warn(clippy::all, clippy::pedantic)]
mod terminal;
mod editor;
use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;

fn main() {
	Editor::default().run();
}
