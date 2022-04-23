use std::io;
use std::net::TcpStream;
use std::thread;

use crate::Notification;

use super::{IoThreads, Message};
use crossbeam_channel::{bounded, Receiver, Sender};

pub struct Transporter(pub Sender<Message>, pub Receiver<Message>, pub IoThreads);

impl Transporter {
    /// Creates an BSP connection via stdio.
    pub fn stdio() -> Self {
        let (writer_sender, writer_receiver) = bounded::<Message>(0);
        let writer = thread::spawn(move || {
            let stdout = io::stdout();
            let mut stdout = stdout.lock();
            writer_receiver
                .into_iter()
                .try_for_each(|it| it.write(&mut stdout))?;
            Ok(())
        });
        let (reader_sender, reader_receiver) = bounded::<Message>(0);
        let reader = thread::spawn(move || {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            while let Some(msg) = Message::read(&mut stdin)? {
                let is_exit = match &msg {
                    Message::Notification(Notification::Exit) => true,
                    _ => false,
                };

                reader_sender.send(msg).unwrap();

                if is_exit {
                    break;
                }
            }
            Ok(())
        });
        let threads = IoThreads { reader, writer };
        Self(writer_sender, reader_receiver, threads)
    }

    /// Creates an BSP connection via socket.
    pub fn socket(stream: TcpStream) -> Self {
        let (reader_receiver, reader) = {
            let stream = stream.try_clone().unwrap();
            let (reader_sender, reader_receiver) = bounded::<Message>(0);
            let reader = thread::spawn(move || {
                let mut buf_read = io::BufReader::new(stream);
                while let Some(msg) = Message::read(&mut buf_read).unwrap() {
                    let is_exit = matches!(&msg, Message::Notification(Notification::Exit));
                    reader_sender.send(msg).unwrap();
                    if is_exit {
                        break;
                    }
                }
                Ok(())
            });
            (reader_receiver, reader)
        };

        let (writer_sender, writer) = {
            let mut stream = stream.try_clone().unwrap();
            let (writer_sender, writer_receiver) = bounded::<Message>(0);
            let writer = thread::spawn(move || {
                writer_receiver
                    .into_iter()
                    .try_for_each(|it| it.write(&mut stream))
                    .unwrap();
                Ok(())
            });
            (writer_sender, writer)
        };

        let io_threads = IoThreads::new(reader, writer);
        Self(writer_sender, reader_receiver, io_threads)
    }
}
