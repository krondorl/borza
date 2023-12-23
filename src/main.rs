use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

// Part 2 Completed
// https://medium.com/@otukof/build-your-text-editor-with-rust-part-2-74e03daef237
//
// Todo: continue development with Part 3
fn main() -> std::result::Result<(), std::io::Error> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => break,
                    _ => {
                        //todo
                    }
                }
                println!("{:?}\r", event);
            }
        } else {
            println!("No input yet\r");
        }
    }
    Ok(())
}
