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
        } else if hid.keys_down().contains(KeyPad::KEY_DDOWN)
            && self.cursor < self.cursor_option_len(versions) - 1
        {
            self.cursor += 1;
        } else if hid.keys_down().contains(KeyPad::KEY_B) {
            if let SubMenu::Main = self.sub_menu {
                self.action = MenuAction::Exit;
            } else {
                self.action = self.actions()[self.actions().len() - 1].clone();
            }
        } else if hid.keys_down().contains(KeyPad::KEY_A) {
            if let SubMenu::Run = self.sub_menu {
                if self.cursor == self.actions().len() as u32 - 1 {
                    self.action = MenuAction::ChangeMenu(SubMenu::Main)
                } else {
                    self.action = MenuAction::Run
                }
            } else {
                self.action = self.actions()[self.cursor as usize].clone()
            }
        }

        match &self.action {
            MenuAction::Exit | MenuAction::Run => return,
            MenuAction::ChangeMenu(c) => {self.sub_menu = *c; self.cursor = 0},
            MenuAction::None => {}
        }
        self.render(console, versions);
    }

    pub fn render(&mut self, console: &Console, versions: &Vec<GameVer>) {
        console.clear();
        println!("{:?}, {}", self.sub_menu, self.cursor)
    }
}
