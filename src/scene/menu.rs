// Menu: Let's Get This Done For The First Release Edition
// Wonder if anything from here will be salvageable

use std::path::PathBuf;

use crate::{
    launcher::GameVer,
    mod_picker,
};
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
    #[cfg(debug_assertions)]
    Log,
}

#[derive(Clone, Debug)]
pub enum MenuAction {
    // All
    None,
    ChangeMenu(SubMenu),
    Exit,
    MoveCursor,

    // Run
    Run,

    // SetUp
    ChangePage(bool),
    SaveConfig,
    ToggleMod,
    ChangeIndex(bool),

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
    const ACTIONS_OPTIONS: [MenuAction; 1] = [MenuAction::ChangeMenu(SubMenu::Main)];

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
    ) {
        self.action = MenuAction::None;

        let mut mod_page = mod_picker::show_page(mods, crate::config(), *page);

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
                    self.action = SubMenu::ACTIONS_SETUP[self.cursor as usize - &mod_page.len()].clone();
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
            }
            //TODO: hold X/Y to scroll faster
            else if hid.keys_down().contains(KeyPad::KEY_DLEFT) {
                self.action = MenuAction::ChangeIndex(false)
            } else if hid.keys_down().contains(KeyPad::KEY_DRIGHT) {
                self.action = MenuAction::ChangeIndex(true)
            }
        }

        match &self.action {
            MenuAction::Exit | MenuAction::Run | MenuAction::None => return,
            MenuAction::ChangeMenu(c) => {
                self.sub_menu = *c;
                self.cursor = 0;
                *page = 0;
            }
            MenuAction::SaveConfig => {
                self.sub_menu = SubMenu::Main;
                self.cursor = 0;
                *page = 0;
            },
            MenuAction::ChangePage(c) => {
                if !c && *page > 0 {
                    *page -= 1;
                } else if *c && *page < mod_picker::num_pages(mods) - 1 {
                    *page += 1;
                }
            }
            MenuAction::ChangeIndex(i) => {
                if let Some(m) = mod_page.get_mut(self.cursor as usize) {
                    let config = crate::config();
                    if m.1 != u16::MAX {
                        config.btks.remove(&m.1);
                        let step: i16 = if *i { 1 } else { -1 };
                        let mut out = m.1.wrapping_add_signed(step);

                        while !mod_picker::is_valid_slot(out) || config.btks.contains_key(&out)  {
                            out = match out.wrapping_add_signed(step) {
                                0x8000..=u16::MAX => 0x10F,
                                0x110..=0x7FFF => 0,
                                c => c,
                            }
                        }

                        config.btks.insert(
                            out,
                            mod_picker::get_mod_name(mods, *page, self.cursor as usize)
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
                        while val <= 0x10F && config.btks.contains_key(&val) {
                            val+=1;
                        }
                        if val <= 0x10F {
                            config.btks.insert(val, mod_picker::get_mod_name(mods, *page, self.cursor as usize));
                        }
                        m.1 = val;
                    } else {
                        config.btks.remove(&m.1);
                        m.1 = u16::MAX;
                    }
                }
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
        )
    }

    pub fn render(
        &mut self,
        console: &Console,
        versions: &Vec<GameVer>,
        mods: &Vec<(String, u16)>,
        page: usize,
        num_pages: usize,
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
                println!("Barista - Set up mods");
                println!();
                if mods.len() == 0 {
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
                    println!();
                    println!("Page {} of {}", page + 1, num_pages);
                    for i in 0..mods.len() {
                        let elmt = &mods[i];
                        println!(
                            "- [{}] {} {}",
                            if self.cursor == i as u32 { "*" } else { " " },
                            match elmt.1 {
                                u16::MAX => "---".to_string(),
                                c => format!("{:03X}", c),
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
                println!("TO BE IMPLEMENTED");
                println!();
                println!(" [{}] Back", if self.cursor == 0 { "*" } else { " " })
            }
            #[cfg(debug_assertions)]
            SubMenu::Log => {
                println!("{}", unsafe {&crate::log::LOG});
            }
        }
    }
}
