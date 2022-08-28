use crate::launcher::GameVer;
use ctru::services::hid::{Hid, KeyPad};

pub struct MenuState {
    pub sub_menu: SubMenu,
    pub cursor: u32,
    pub action: MenuAction,
}

pub enum SubMenu {
    Main,
    Run,
    Options,
    Music,
    SetUp,
}

/*
Run game
    Version 1
    Version 2
    ...
Set up minigames
    TODO
Music
    TODO
Options
    TODO
*/

pub enum MenuAction {
    None,
    ChangeMenu(SubMenu),
    ChooseVersionByPos,
    ChooseVersion(GameVer),
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

impl MenuState {
    const ACTIONS_MAIN: [MenuAction; 5] = [
        MenuAction::ChangeMenu(SubMenu::Run),
        MenuAction::ChangeMenu(SubMenu::SetUp),
        MenuAction::ChangeMenu(SubMenu::Music),
        MenuAction::ChangeMenu(SubMenu::Options),
        MenuAction::Exit,
    ];
    const ACTIONS_RUN: [MenuAction; 2] = [
        MenuAction::ChooseVersionByPos,
        MenuAction::ChangeMenu(SubMenu::Main),
    ];
    const ACTIONS_SETUP: [MenuAction; 1] = [MenuAction::ChangeMenu(SubMenu::Main)];
    const ACTIONS_MUSIC: [MenuAction; 1] = [MenuAction::ChangeMenu(SubMenu::Main)];
    const ACTIONS_OPTIONS: [MenuAction; 1] = [MenuAction::ChangeMenu(SubMenu::Main)];

    pub fn run(&mut self, hid: Hid, versions: Vec<GameVer>) {
        if hid.keys_down().contains(KeyPad::KEY_START) {
            self.action = MenuAction::Exit;
            return;
        }
    }

    pub fn render(&mut self) {}
}
