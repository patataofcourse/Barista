use crate::format::bcstm::BCSTMFile;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

pub struct AudioManager {
    tx: Sender<AudioMessage>,
    is_loaded: bool,
    is_playing: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || audio_main(rx));
        Self {
            tx,
            is_loaded: false,
            is_playing: false,
        }
    }

    pub fn load(&mut self, file: String) {
        self.is_loaded = true;
        self.tx.send(AudioMessage::LoadFile(file)).unwrap();
    }

    pub fn play(&mut self) {
        self.is_playing = self.is_loaded;
        self.tx.send(AudioMessage::Play).unwrap();
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
        self.tx.send(AudioMessage::Play).unwrap();
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.is_loaded = false;
        self.tx.send(AudioMessage::Unload).unwrap();
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        self.tx.send(AudioMessage::Exit).unwrap();
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

    loop {
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
