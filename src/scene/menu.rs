// Menu: Let's Get This Done For The First Release Edition
// Wonder if anything from here will be salvageable

use std::path::PathBuf;

use crate::{format::barista_cfg::BaristaConfig, launcher::GameVer, mod_picker};
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

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SubMenu {
    Main,
    Run,
    Options,
    Music,
    SetUp,
    #[cfg(debug_assertions)]
    Log,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MenuAction {
    // All
    None,
    ChangeMenu(SubMenu),
    Exit,
    MoveCursor,

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
    #[cfg(not(feature = "audio"))]
    const ACTIONS_MUSIC: [MenuAction; 1] = [MenuAction::ChangeMenu(SubMenu::Main)];
    const ACTIONS_OPTIONS: [MenuAction; 2] =
        [MenuAction::ToggleSetting(0), MenuAction::SaveSettings];

    pub fn actions(&self) -> &[MenuAction] {
        match &self {
            SubMenu::Main => &Self::ACTIONS_MAIN,
            SubMenu::Run => &Self::ACTIONS_RUN,
            SubMenu::SetUp => &Self::ACTIONS_SETUP,
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
            } else if let SubMenu::SetUp = self {
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

    pub fn run(
        &mut self,
        hid: &Hid,
        console: &Console,
        versions: &Vec<GameVer>,
        mods: &Vec<PathBuf>,
        page: &mut usize,
        settings: &mut BaristaConfig,
    ) {
        self.action = MenuAction::None;

        let mut mod_page = if self.sub_menu == SubMenu::SetUp {
            mod_picker::show_page(mods, crate::config(), *page)
        } else {
            vec![]
        };

        if hid.keys_down().contains(KeyPad::KEY_START) {
            self.action = MenuAction::Exit;
            return;
        }

        if hid.keys_down().contains(KeyPad::KEY_DUP) {
            if self.cursor > 0 {
                self.cursor -= 1;
            } else {
                self.cursor = self.cursor_option_len(versions, &mod_page) - 1;
            }
            self.action = MenuAction::MoveCursor
        } else if hid.keys_down().contains(KeyPad::KEY_DDOWN) {
            if self.cursor < self.cursor_option_len(versions, &mod_page) - 1 {
                self.cursor += 1;
            } else {
                self.cursor = 0;
            }
            self.action = MenuAction::MoveCursor
        } else if hid.keys_down().contains(KeyPad::KEY_B) {
            if let SubMenu::Main = self.sub_menu {
                self.action = MenuAction::Exit;
            } else {
                self.action = self.actions()[self.actions().len() - 1].clone();
            }
        } else if hid.keys_down().contains(KeyPad::KEY_A) {
            if let SubMenu::Run = self.sub_menu {
                if self.cursor == self.cursor_option_len(versions, &mod_page) - 1 {
                    self.action = MenuAction::ChangeMenu(SubMenu::Main)
                } else {
                    self.action = MenuAction::Run
                }
            } else if let SubMenu::SetUp = self.sub_menu {
                if self.cursor_option_len(versions, &mod_page) - self.cursor <= 3 {
                    self.action =
                        SubMenu::ACTIONS_SETUP[self.cursor as usize - mod_page.len()].clone();
                } else {
                    self.action = MenuAction::ToggleMod;
                }
            } else {
                self.action = self.actions()[self.cursor as usize].clone()
            }
        }
        #[cfg(debug_assertions)]
        if hid.keys_down().contains(KeyPad::KEY_SELECT) {
            self.action = MenuAction::ChangeMenu(SubMenu::Log)
        }
        if let SubMenu::SetUp = &self.sub_menu {
            if hid.keys_down().contains(KeyPad::KEY_L) {
                self.action = MenuAction::ChangePage(false)
            } else if hid.keys_down().contains(KeyPad::KEY_R) {
                self.action = MenuAction::ChangePage(true)
            } else if hid.keys_down().contains(KeyPad::KEY_DLEFT) {
                if hid.keys_held().contains(KeyPad::KEY_X) {
                    self.action = MenuAction::ChangeIndex(false, true)
                } else {
                    self.action = MenuAction::ChangeIndex(false, false)
                }
            } else if hid.keys_down().contains(KeyPad::KEY_DRIGHT) {
                if hid.keys_held().contains(KeyPad::KEY_X) {
                    self.action = MenuAction::ChangeIndex(true, true)
                } else {
                    self.action = MenuAction::ChangeIndex(true, false)
                }
            }
        }

        match &self.action {
            MenuAction::Exit | MenuAction::Run | MenuAction::None => return,
            MenuAction::ChangeMenu(c) => {
                if *c == SubMenu::SetUp {
                    mod_page = mod_picker::show_page(mods, crate::config(), *page);
                }

                self.sub_menu = *c;
                self.cursor = 0;
                *page = 0;
            }
            MenuAction::SaveConfig | MenuAction::SaveSettings => {
                self.sub_menu = SubMenu::Main;
                self.cursor = 0;
                *page = 0;
            }
            MenuAction::ChangePage(c) => {
                if !c && *page > 0 {
                    *page -= 1;
                } else if *c && *page < mod_picker::num_pages(mods) - 1 {
                    *page += 1;
                }
            }
            //TODO: properly order stuff in new gate mode (both ChangeIndex and ToggleMod)
            MenuAction::ChangeIndex(i, fast) => {
                if let Some(m) = mod_page.get_mut(self.cursor as usize) {
                    let config = crate::config();
                    if m.1 != u16::MAX {
                        config.btks.remove(&m.1);
                        let mut step: i16 = if *i { 1 } else { -1 };
                        if *fast {
                            step *= 0x10
                        }
                        let mut out = m.1.wrapping_add_signed(step);

                        while !mod_picker::is_valid_slot(out) || config.btks.contains_key(&out) {
                            out = match out.wrapping_add_signed(step) {
                                0x8000..=u16::MAX => 0x113,
                                0x114..=0x7FFF => 0,
                                c => c,
                            }
                        }

                        config.btks.insert(
                            out,
                            mod_picker::get_mod_name(mods, *page, self.cursor as usize),
                        );
                        m.1 = out;
                    }
                }
            }
            MenuAction::ToggleMod => {
                if let Some(m) = mod_page.get_mut(self.cursor as usize) {
                    let config = crate::config();
                    if m.1 == u16::MAX {
                        let mut val = 0;
                        while val <= 0x113 && config.btks.contains_key(&val) {
                            val += 1;
                        }
                        if val <= 0x113 {
                            config.btks.insert(
                                val,
                                mod_picker::get_mod_name(mods, *page, self.cursor as usize),
                            );
                        } else {
                            val = u16::MAX;
                        }
                        m.1 = val;
                    } else {
                        config.btks.remove(&m.1);
                        m.1 = u16::MAX;
                    }
                }
            }
            MenuAction::ToggleSetting(c) => match c {
                0 => {
                    settings.original_gates = !settings.original_gates;
                }
                1 => {} // so clippy shuts up
                _ => {}
            },
            MenuAction::MoveCursor => {}
            #[cfg(feature = "audio")]
            MenuAction::ToggleAudio => {}
        }
        self.render(
            console,
            versions,
            &mod_page,
            *page,
            mod_picker::num_pages(mods),
            settings,
        )
    }

    pub fn render(
        &mut self,
        console: &Console,
        versions: &Vec<GameVer>,
        mods: &Vec<(String, u16)>,
        page: usize,
        num_pages: usize,
        settings: &BaristaConfig,
    ) {
        console.clear();
        match &self.sub_menu {
            SubMenu::Main => {
                println!("Barista - Main menu");
                println!();
                println!("Controls:");
                println!("- DPad up/down: move cursor");
                println!("- A: choose selected option");
                println!("- B: go to prev menu");
                println!("- Start: exit Barista");
                #[cfg(debug_assertions)]
                println!("- Select: open debug log");
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
                for (vnum, ver) in versions.iter().enumerate() {
                    println!(
                        " [{}] {}",
                        if self.cursor as usize == vnum {
                            "*"
                        } else {
                            " "
                        },
                        ver
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
                println!("Barista - Set up mods");
                println!();
                if mods.is_empty() {
                    println!(
                        "Put some mods in your /spicerack/mods\nfolder in order to load them!"
                    );
                    println!();
                    println!("- [*] Back")
                } else {
                    println!("Choose what mods to load with Saltwater");
                    println!("Disabled mods show index --- instead");
                    println!();
                    println!("L/R buttons or Prev/Next to change page");
                    println!("DPad Left/Right to change index");
                    println!("Hold X to scroll indexes faster");
                    println!();
                    println!("Page {} of {}", page + 1, num_pages);
                    for (i, elmt) in mods.iter().enumerate() {
                        println!(
                            "- [{}] {} {}",
                            if self.cursor == i as u32 { "*" } else { " " },
                            match elmt.1 {
                                u16::MAX => "---".to_string(),
                                c =>
                                    if c >= 0x100 && !settings.original_gates {
                                        format!(
                                            "G{}{}",
                                            if c >= 0x110 { c & 3 } else { (c & 0xFF) >> 2 },
                                            if c >= 0x110 {
                                                "P".to_string()
                                            } else if c & 3 == 3 {
                                                "E".to_string()
                                            } else {
                                                (c & 3).to_string()
                                            }
                                        )
                                    } else {
                                        format!("{:03X}", c)
                                    },
                            },
                            elmt.0
                        );
                    }
                    println!();
                    println!(
                        "- [{}] Prev",
                        if self.cursor == self.cursor_option_len(versions, mods) - 3 {
                            "*"
                        } else {
                            " "
                        }
                    );
                    println!(
                        "- [{}] Next",
                        if self.cursor == self.cursor_option_len(versions, mods) - 2 {
                            "*"
                        } else {
                            " "
                        }
                    );
                    println!();
                    println!(
                        "- [{}] Back",
                        if self.cursor == self.cursor_option_len(versions, mods) - 1 {
                            "*"
                        } else {
                            " "
                        }
                    );
                }
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
                println!(
                    " [{}] ({}) Use 0x100 format for gates",
                    if self.cursor == 0 { "*" } else { " " },
                    if settings.original_gates { "x" } else { " " }
                );
                println!();
                println!(" [{}] Back", if self.cursor == 1 { "*" } else { " " })
            }
            #[cfg(debug_assertions)]
            SubMenu::Log => {
                println!("{}", unsafe { &crate::log::LOG });
            }
        }
    }
}
