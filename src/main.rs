mod frust;
use frust::tui::App;
use std::io;

fn main() -> io::Result<()> {
    let mut app = App::default();
    app.run()?;
    app.clear()
}
