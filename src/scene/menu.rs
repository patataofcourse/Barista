use std::ffi::OsStr;

use crate::{launcher::GameVer, Result};
use ctru::{
    console::Console,
    services::{
        fs::{self, Fs},
        hid::{Hid, KeyPad},
    },
};

#[derive(Clone, Debug)]
pub struct MenuState {
    pub sub_menu: SubMenu,
    pub cursor: u32,
    pub action: MenuAction,
}

#[derive(Clone, Debug, Copy)]
pub enum SubMenu {
    Main,
    Run,
    Options,
    Music,
    SetUp,
}

#[derive(Clone, Debug)]
pub enum MenuAction {
    None,
    ChangeMenu(SubMenu),
    Run,
    Exit,
    MoveCursor,
    #[cfg(feature = "audio")]
    ToggleAudio,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            sub_menu: SubMenu::Main,
            cursor: 0,
            action: MenuAction::None,
        }
    }
}

impl SubMenu {
    const ACTIONS_MAIN: [MenuAction; 5] = [
        MenuAction::ChangeMenu(SubMenu::Run),
        MenuAction::ChangeMenu(SubMenu::SetUp),
        MenuAction::ChangeMenu(SubMenu::Music),
        MenuAction::ChangeMenu(SubMenu::Options),
        MenuAction::Exit,
    ];
    const ACTIONS_RUN: [MenuAction; 2] = [MenuAction::Run, MenuAction::ChangeMenu(SubMenu::Main)];
    const ACTIONS_SETUP: [MenuAction; 1] = [MenuAction::ChangeMenu(SubMenu::Main)];
    #[cfg(feature = "audio")]
    const ACTIONS_MUSIC: [MenuAction; 2] = [
        MenuAction::ToggleAudio,
        MenuAction::ChangeMenu(SubMenu::Main),
    ];
    #[cfg(not(feature = "audio"))]
    const ACTIONS_MUSIC: [MenuAction; 1] = [MenuAction::ChangeMenu(SubMenu::Main)];
    const ACTIONS_OPTIONS: [MenuAction; 1] = [MenuAction::ChangeMenu(SubMenu::Main)];

    pub const fn actions(&self) -> &[MenuAction] {
        match &self {
            SubMenu::Main => &Self::ACTIONS_MAIN,
            SubMenu::Run => &Self::ACTIONS_RUN,
            SubMenu::SetUp => &Self::ACTIONS_SETUP,
            SubMenu::Music => &Self::ACTIONS_MUSIC,
            SubMenu::Options => &Self::ACTIONS_OPTIONS,
        }
    }

    pub fn cursor_option_len(&self, versions: &Vec<GameVer>) -> u32 {
        (self.actions().len()
            + if let SubMenu::Run = self {
                versions.len() - 1
            } else {
                0
            }) as u32
    }
}

impl MenuState {
    pub const fn actions(&self) -> &[MenuAction] {
        self.sub_menu.actions()
    }
    pub fn cursor_option_len(&self, versions: &Vec<GameVer>) -> u32 {
        self.sub_menu.cursor_option_len(versions)
    }

    pub fn run(&mut self, hid: &Hid, console: &Console, versions: &Vec<GameVer>) -> Result<()> {
        self.action = MenuAction::None;

        if hid.keys_down().contains(KeyPad::KEY_START) {
            self.action = MenuAction::Exit;
            return Ok(());
        }

        if hid.keys_down().contains(KeyPad::KEY_DUP) && self.cursor > 0 {
            self.cursor -= 1;
            self.action = MenuAction::MoveCursor
        } else if hid.keys_down().contains(KeyPad::KEY_DDOWN)
            && self.cursor < self.cursor_option_len(versions) - 1
        {
            self.cursor += 1;
            self.action = MenuAction::MoveCursor
        } else if hid.keys_down().contains(KeyPad::KEY_B) {
            if let SubMenu::Main = self.sub_menu {
                self.action = MenuAction::Exit;
            } else {
                self.action = self.actions()[self.actions().len() - 1].clone();
            }
        } else if hid.keys_down().contains(KeyPad::KEY_A) {
            if let SubMenu::Run = self.sub_menu {
                if self.cursor == self.cursor_option_len(versions) - 1 {
                    self.action = MenuAction::ChangeMenu(SubMenu::Main)
                } else {
                    self.action = MenuAction::Run
                }
            } else {
                self.action = self.actions()[self.cursor as usize].clone()
            }
        }

        match &self.action {
            MenuAction::Exit | MenuAction::Run | MenuAction::None => return Ok(()),
            MenuAction::ChangeMenu(c) => {
                self.sub_menu = *c;
                self.cursor = 0
            }
            MenuAction::MoveCursor => {}
            #[cfg(feature = "audio")]
            MenuAction::ToggleAudio => {}
        }
        self.render(console, versions)
    }

    pub fn render(&mut self, console: &Console, versions: &Vec<GameVer>) -> Result<()> {
        console.clear();
        match &self.sub_menu {
            SubMenu::Main => {
                println!("Barista - Main menu");
                println!();
                println!("Controls:");
                println!("- DPad up/down: move cursor");
                println!("- A: choose selected option");
                println!("- B: go to prev menu");
                println!("- Start - exit Barista");
                println!();
                println!(
                    " [{}] Run Saltwater",
                    if self.cursor == 0 { "*" } else { " " }
                );
                println!(
                    " [{}] Set up mods",
                    if self.cursor == 1 { "*" } else { " " }
                );
                println!(" [{}] Music", if self.cursor == 2 { "*" } else { " " });
                println!(" [{}] Settings", if self.cursor == 3 { "*" } else { " " });
                println!(
                    " [{}] Exit Barista",
                    if self.cursor == 4 { "*" } else { " " }
                );
            }
            SubMenu::Run => {
                println!("Barista - Run Saltwater");
                println!();
                println!("Choose a version to run with Saltwater");
                println!();
                for vnum in 0..versions.len() {
                    println!(
                        " [{}] {}",
                        if self.cursor as usize == vnum {
                            "*"
                        } else {
                            " "
                        },
                        versions[vnum]
                    );
                }
                println!(
                    " [{}] Back",
                    if self.cursor as usize == versions.len() {
                        "*"
                    } else {
                        " "
                    }
                );
            }
            SubMenu::SetUp => {
                let fs = Fs::init()?;
                println!("Barista - Set up mods");
                println!();
                println!("TO BE IMPLEMENTED");
                println!("{:?}", crate::config().btks);
                println!("{:?}", {
                    //TODO: do this ONCE and that's it
                    let mut v = vec![];
                    let mut sdmc = fs.sdmc()?;
                    let mut iter = fs::read_dir(&sdmc, "/spicerack/mods")?;
                    for f in iter {
                        let f = f?;
                        let path = f.path();
                        if path.as_path().extension() == Some(&OsStr::new("btk")) && f.metadata()?.is_file() {
                            v.push(path);
                        }
                    }
                    v
                });
                println!();
                println!(" [{}] Back", if self.cursor == 0 { "*" } else { " " })
            }
            SubMenu::Music => {
                println!("Barista - Music");
                println!();
                let pos;
                #[cfg(feature = "audio")]
                {
                    pos = 1;
                    println!("Current status: very broken");
                    println!();
                    println!(
                        " [{}] {}",
                        if self.cursor == 0 { "*" } else { " " },
                        if crate::audio().is_playing() {
                            "Disable"
                        } else {
                            "Enable"
                        }
                    );
                }
                #[cfg(not(feature = "audio"))]
                {
                    pos = 0;
                    println!("TO BE IMPLEMENTED\n");
                }
                println!(" [{}] Back", if self.cursor == pos { "*" } else { " " })
            }
            SubMenu::Options => {
                println!("Barista - Settings");
                println!();
                println!("TO BE IMPLEMENTED");
                println!();
                println!(" [{}] Back", if self.cursor == 0 { "*" } else { " " })
            }
        }
        Ok(())
    }
}
