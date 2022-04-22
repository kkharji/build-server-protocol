use std::{io, thread};

pub struct IoThreads {
    pub reader: thread::JoinHandle<io::Result<()>>,
    pub writer: thread::JoinHandle<io::Result<()>>,
}

impl IoThreads {
    // Creates an IoThreads
    pub(crate) fn new(
        reader: thread::JoinHandle<io::Result<()>>,
        writer: thread::JoinHandle<io::Result<()>>,
    ) -> IoThreads {
        IoThreads { reader, writer }
    }

    pub fn join(self) -> io::Result<()> {
        match self.reader.join() {
            Ok(r) => r?,
            Err(err) => {
                println!("reader panicked!");
                std::panic::panic_any(err)
            }
        }
        match self.writer.join() {
            Ok(r) => r,
            Err(err) => {
                println!("writer panicked!");
                std::panic::panic_any(err);
            }
        }
    }
}
