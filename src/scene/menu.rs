use crate::launcher::GameVer;
use ctru::{
    console::Console,
    services::hid::{Hid, KeyPad},
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

    pub fn run(&mut self, hid: &Hid, console: &Console, versions: &Vec<GameVer>) {
        self.action = MenuAction::None;

        if hid.keys_down().contains(KeyPad::KEY_START) {
            self.action = MenuAction::Exit;
            return;
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
            MenuAction::Exit | MenuAction::Run | MenuAction::None => return,
            MenuAction::ChangeMenu(c) => {
                self.sub_menu = *c;
                self.cursor = 0
            }
            MenuAction::MoveCursor => {}
        }
        self.render(console, versions);
    }

    pub fn render(&mut self, console: &Console, versions: &Vec<GameVer>) {
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
                println!(" [{}] Run Saltwater", if self.cursor == 0 {"*"} else {" "});
                println!(" [{}] Set up mods", if self.cursor == 1 {"*"} else {" "});
                println!(" [{}] Music", if self.cursor == 2 {"*"} else {" "});
                println!(" [{}] Settings", if self.cursor == 3 {"*"} else {" "});
                println!(" [{}] Exit Barista", if self.cursor == 4 {"*"} else {" "});
            }
            SubMenu::Run => {
                println!("Barista - Run Saltwater");
                println!();
                println!("Choose a version to run with Saltwater");
                println!();
                for vnum in 0..versions.len() {
                    println!(" [{}] {}", if self.cursor as usize == vnum {"*"} else {" "}, versions[vnum]);
                }
                println!(" [{}] Back", if self.cursor as usize == versions.len() {"*"} else {" "});
            }
            SubMenu::SetUp => {
                println!("Barista - Set up mods");
                println!();
                println!("TO BE IMPLEMENTED");
                println!();
                println!(" [{}] Back", if self.cursor == 0 {"*"} else {" "})
            }
            SubMenu::Music => {
                println!("Barista - Music");
                println!();
                println!("TO BE IMPLEMENTED");
                println!();
                println!(" [{}] Back", if self.cursor == 0 {"*"} else {" "})
            }
            SubMenu::Options => {
                println!("Barista - Settings");
                println!();
                println!("TO BE IMPLEMENTED");
                println!();
                println!(" [{}] Back", if self.cursor == 0 {"*"} else {" "})
            }
        }
    }
}
