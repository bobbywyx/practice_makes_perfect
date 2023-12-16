use editor::Editor;
mod renderer;

fn main() {
    if let Err(e) = Editor::new().run() {
        println!("{}", e);
        std::process::exit(1);
    }
}
