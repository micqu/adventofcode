use std::{
    fs::File,
    io::{self, prelude::*},
    path::Path,
    rc::Rc,
};

pub struct Reader {
    reader: io::BufReader<File>,
    buf: Rc<String>,
}

fn new_buf() -> Rc<String> {
    Rc::new(String::with_capacity(1024)) // Tweakable capacity
}

impl Reader {
    pub fn load_input(file: &str) -> io::Result<Self> {
        let path = std::env::current_dir()
            .unwrap()
            .join(Path::new(file));
        Self::open(path)
    }

    pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let buf = new_buf();
        Ok(Self { reader, buf })
    }

    pub fn read_line<'buf>(&mut self, buffer: &'buf mut String)
    -> Option<io::Result<&'buf mut String>> {
        buffer.clear();

        self.reader
            .read_line(buffer)
            .map(|u| if u == 0 { None } else { Some(buffer) })
            .transpose()
    }
    
    pub fn read<'buf>(&mut self, buffer: &'buf mut String) {
        self.reader
            .read_to_string(buffer)
            .expect("failed to read file");
    }
}

impl Iterator for Reader {
    type Item = io::Result<Rc<String>>;

    fn next(&mut self) -> Option<Self::Item> {
        let buf = match Rc::get_mut(&mut self.buf) {
            Some(buf) => {
                buf.clear();
                buf
            }
            None => {
                self.buf = new_buf();
                Rc::make_mut(&mut self.buf)
            }
        };

        self.reader
            .read_line(buf)
            .map(|u| {
                if u == 0 {
                    None
                } else {
                    Some(Rc::clone(&self.buf))
                }
            })
            .transpose()
    }
}