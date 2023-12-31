use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, queue, terminal};
use std::io::{self, stdout, Write};
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        Output::clear_screen().expect("Error");
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
    output: Output,
}

impl Editor {
    fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
        }
    }

    fn process_keypress(&self) -> std::io::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                kind: _,
                state: _,
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    fn run(&mut self) -> std::io::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}

struct Output {
    win_size: (usize, usize),
    editor_contents: EditorContents,
}

impl Output {
    fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            editor_contents: EditorContents::new(),
        }
    }

    fn clear_screen() -> std::io::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        for i in 0..screen_rows {
            self.editor_contents.push('~');
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n");
            }
        }
    }

    fn refresh_screen(&mut self) -> std::io::Result<()> {
        queue!(
            self.editor_contents,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        self.draw_rows();
        queue!(self.editor_contents, cursor::MoveTo(0, 0))?;
        self.editor_contents.flush()
    }
}

struct EditorContents {
    content: String,
}

impl EditorContents {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn push(&mut self, ch: char) {
        self.content.push(ch);
    }

    fn push_str(&mut self, string: &str) {
        self.content.push_str(string);
    }
}

impl io::Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

// Continue development with Part 3
// https://medium.com/@otukof/b030670fa815
// just before "Hide The Cursor When Repainting"
fn main() -> std::io::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
