use crate::format::bcstm::BCSTMFile;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

pub struct AudioManager(Sender<AudioMessage>);

impl AudioManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || audio_main(rx));
        Self(tx)
    }

    pub fn load(&mut self, file: String) {
        self.0.send(AudioMessage::LoadFile(file)).unwrap();
    }

    pub fn play(&mut self) {
        self.0.send(AudioMessage::Play).unwrap();
    }

    pub fn pause(&mut self) {
        self.0.send(AudioMessage::Play).unwrap();
    }

    pub fn stop(&mut self) {
        self.0.send(AudioMessage::Unload).unwrap();
    }
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        self.0.send(AudioMessage::Exit).unwrap();
    }
}

struct AudioContext {
    pub bcstm: Option<BCSTMFile>,
    pub playing: bool,
    pub rx: Receiver<AudioMessage>,
}

enum AudioMessage {
    LoadFile(String),
    Play,
    Pause,
    Exit,
    Unload,
    None,
}

fn audio_main(rx: Receiver<AudioMessage>) {
    let mut ctx = AudioContext {
        bcstm: None,
        playing: false,
        rx,
    };
    
    loop{

        let msg = ctx
            .rx
            .recv_timeout(Duration::from_millis(20))
            .unwrap_or(AudioMessage::None);

        match msg {
            AudioMessage::LoadFile(c) => ctx.bcstm = Some(BCSTMFile::open_from_file(c).unwrap()),
            AudioMessage::Play => {
                if let Some(ref mut c) = ctx.bcstm {
                    c.play()
                }
            }
            AudioMessage::Pause => {
                if let Some(ref mut c) = ctx.bcstm {
                    c.pause()
                }
            }
            AudioMessage::Unload => ctx.bcstm = None,
            AudioMessage::Exit => break,
            AudioMessage::None => (),
        }

        if let Some(ref mut c) = ctx.bcstm {
            c.tick().unwrap(); //TODO: test value, stop if false
        }
    }
}
