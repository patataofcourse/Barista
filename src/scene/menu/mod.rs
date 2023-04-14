// Menu: Let's Get This Done For The First Release Edition
// Wonder if anything from here will be salvageable

use crate::launcher::GameVer;
use ctru::services::hid::KeyPad;

pub mod render;
pub mod run;

#[derive(Clone, Debug)]
pub struct MenuState {
    pub sub_menu: SubMenu,
    pub cursor: u32,
    pub action: MenuAction,
    pub hold_controller: HoldController,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct HoldController {
    pub up: Option<u32>,
    pub down: Option<u32>,
    pub left: Option<u32>,
    pub right: Option<u32>,
}

impl HoldController {
    const FIRST_PRESS_TIME: u32 = 15;
    const LOOP_PRESS_TIME: u32 = 4;

    pub fn update(&mut self, keys: KeyPad) {
        if keys.contains(KeyPad::DUP) {
            if let Some(c) = &mut self.up {
                *c += 1;
            } else {
                self.up = Some(0)
            }
        } else {
            self.up = None;
        }
        if keys.contains(KeyPad::DDOWN) {
            if let Some(c) = &mut self.down {
                *c += 1;
            } else {
                self.down = Some(0)
            }
        } else {
            self.down = None;
        }
        if keys.contains(KeyPad::DLEFT) {
            if let Some(c) = &mut self.left {
                *c += 1;
            } else {
                self.left = Some(0)
            }
        } else {
            self.left = None;
        }
        if keys.contains(KeyPad::DRIGHT) {
            if let Some(c) = &mut self.right {
                *c += 1;
            } else {
                self.right = Some(0)
            }
        } else {
            self.right = None;
        }
    }

    pub fn should_click(&self, key: KeyPad) -> bool {
        let check = |t| t == 0 || (t >= Self::FIRST_PRESS_TIME && t % Self::LOOP_PRESS_TIME == 0);

        if key == KeyPad::DUP {
            if let Some(t) = self.up {
                check(t)
            } else {
                false
            }
        } else if key == KeyPad::DDOWN {
            if let Some(t) = self.down {
                check(t)
            } else {
                false
            }
        } else if key == KeyPad::DLEFT {
            if let Some(t) = self.left {
                check(t)
            } else {
                false
            }
        } else if key == KeyPad::DRIGHT {
            if let Some(t) = self.right {
                check(t)
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SubMenu {
    Main,
    Run,
    Options,
    #[cfg(feature = "audio")]
    Music,
    SetUp(bool),
    #[cfg(debug_assertions)]
    Log,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MenuAction {
    // All
    None,
    ChangeMenu(SubMenu),
    Exit,
    UpdateScreen,

    // Run
    Run,

    // Options
    ToggleSetting(u8),
    SaveSettings,

    // SetUp
    ChangePage(bool),
    SaveConfig,
    ToggleMod,
    ChangeIndex(bool, bool),

    // Music
    #[cfg(feature = "audio")]
    ToggleAudio,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            sub_menu: SubMenu::Main,
            cursor: 0,
            action: MenuAction::None,
            hold_controller: HoldController::default(),
        }
    }
}

impl SubMenu {
    const ACTIONS_MAIN: &[MenuAction] = &[
        MenuAction::ChangeMenu(SubMenu::Run),
        MenuAction::ChangeMenu(SubMenu::SetUp(false)),
        #[cfg(feature = "audio")]
        MenuAction::ChangeMenu(SubMenu::Music),
        MenuAction::ChangeMenu(SubMenu::Options),
        MenuAction::Exit,
    ];
    const ACTIONS_RUN: [MenuAction; 1] = [MenuAction::ChangeMenu(SubMenu::Main)];
    const ACTIONS_SETUP: [MenuAction; 3] = [
        MenuAction::ChangePage(false),
        MenuAction::ChangePage(true),
        MenuAction::SaveConfig,
    ];
    #[cfg(feature = "audio")]
    const ACTIONS_MUSIC: [MenuAction; 2] = [
        MenuAction::ToggleAudio,
        MenuAction::ChangeMenu(SubMenu::Main),
    ];
    const ACTIONS_OPTIONS: [MenuAction; 3] = [
        MenuAction::ToggleSetting(0),
        MenuAction::ToggleSetting(1),
        MenuAction::SaveSettings,
    ];

    pub fn actions(&self) -> &[MenuAction] {
        match &self {
            SubMenu::Main => Self::ACTIONS_MAIN,
            SubMenu::Run => &Self::ACTIONS_RUN,
            SubMenu::SetUp(_) => &Self::ACTIONS_SETUP,
            #[cfg(feature = "audio")]
            SubMenu::Music => &Self::ACTIONS_MUSIC,
            SubMenu::Options => &Self::ACTIONS_OPTIONS,
            #[cfg(debug_assertions)]
            SubMenu::Log => &[MenuAction::ChangeMenu(SubMenu::Main)],
        }
    }

    pub fn cursor_option_len(&self, versions: &Vec<GameVer>, mods: &Vec<(String, u16)>) -> u32 {
        (self.actions().len()
            + if let SubMenu::Run = self {
                versions.len()
            } else if let SubMenu::SetUp(_) = self {
                mods.len()
            } else {
                0
            }) as u32
    }
}

impl MenuState {
    pub fn actions(&self) -> &[MenuAction] {
        self.sub_menu.actions()
    }
    pub fn cursor_option_len(&self, versions: &Vec<GameVer>, mods: &Vec<(String, u16)>) -> u32 {
        self.sub_menu.cursor_option_len(versions, mods)
    }
}
