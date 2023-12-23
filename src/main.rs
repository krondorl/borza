use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};
use std::io;
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

struct Reader;

impl Reader {
    fn read_key(&self) -> std::io::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

struct Editor {
    reader: Reader,
}

impl Editor {
    fn new() -> Self {
        Self { reader: Reader }
    }

    fn process_keypress(&self) -> std::io::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                kind,
                state,
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    fn run(&self) -> std::io::Result<bool> {
        self.process_keypress()
    }
}

// Continue development with Part 3
// https://medium.com/@otukof/b030670fa815
// "Now we have a simple main() function and we would like to keep it as such."
fn main() -> std::io::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
