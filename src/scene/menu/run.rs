use std::path::PathBuf;

use ctru::{console::Console, services::hid::KeyPad, services::Hid};

use crate::{
    format::barista_cfg::{BaristaConfig, SlotTitleMode},
    launcher::GameVer,
    mod_picker,
};

use super::{MenuAction, MenuState, SubMenu};

impl MenuState {
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

        let mut mod_page = if let SubMenu::SetUp(_) = self.sub_menu {
            mod_picker::show_page(mods, crate::config(), *page)
        } else {
            vec![]
        };

        if hid.keys_down().contains(KeyPad::START) {
            self.action = MenuAction::Exit;
            return;
        }

        self.hold_controller.update(hid.keys_held());

        if self.hold_controller.should_click(KeyPad::DUP) {
            if self.cursor > 0 {
                self.cursor -= 1;
            } else {
                self.cursor = self.cursor_option_len(versions, &mod_page) - 1;
            }
            self.action = MenuAction::UpdateScreen
        } else if self.hold_controller.should_click(KeyPad::DDOWN) {
            if self.cursor < self.cursor_option_len(versions, &mod_page) - 1 {
                self.cursor += 1;
            } else {
                self.cursor = 0;
            }
            self.action = MenuAction::UpdateScreen
        } else if hid.keys_down().contains(KeyPad::B) {
            if let SubMenu::Main = self.sub_menu {
                self.action = MenuAction::Exit;
            } else {
                self.action = self.actions()[self.actions().len() - 1].clone();
            }
        } else if hid.keys_down().contains(KeyPad::A) {
            if let SubMenu::Run = self.sub_menu {
                if self.cursor == self.cursor_option_len(versions, &mod_page) - 1 {
                    self.action = MenuAction::ChangeMenu(SubMenu::Main)
                } else {
                    self.action = MenuAction::Run
                }
            } else if let SubMenu::SetUp(_) = self.sub_menu {
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
        if hid.keys_down().contains(KeyPad::SELECT) {
            self.action = MenuAction::ChangeMenu(SubMenu::Log)
        }
        if let SubMenu::SetUp(c) = &mut self.sub_menu {
            if hid.keys_down().contains(KeyPad::Y) {
                *c = !*c;
                self.action = MenuAction::UpdateScreen
            }
            if hid.keys_down().contains(KeyPad::L) {
                self.action = MenuAction::ChangePage(false)
            } else if hid.keys_down().contains(KeyPad::R) {
                self.action = MenuAction::ChangePage(true)
            } else if self.hold_controller.should_click(KeyPad::DLEFT) {
                if hid.keys_held().contains(KeyPad::X) {
                    self.action = MenuAction::ChangeIndex(false, true)
                } else {
                    self.action = MenuAction::ChangeIndex(false, false)
                }
            } else if self.hold_controller.should_click(KeyPad::DRIGHT) {
                if hid.keys_held().contains(KeyPad::X) {
                    self.action = MenuAction::ChangeIndex(true, true)
                } else {
                    self.action = MenuAction::ChangeIndex(true, false)
                }
            }
        }

        match &self.action {
            MenuAction::Exit | MenuAction::Run | MenuAction::None => return,
            MenuAction::ChangeMenu(c) => {
                if let SubMenu::SetUp(_) = *c {
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
                let old_len = mod_page.len() as u32;
                mod_page = mod_picker::show_page(mods, crate::config(), *page);
                log!(General, "{} {} {}", old_len, mod_page.len(), self.cursor);

                // Make sure the cursor is in-bounds
                if self.cursor < old_len {
                    self.cursor = self.cursor.clamp(0, mod_page.len() as u32 - 1);
                } else {
                    self.cursor = self
                        .cursor
                        .wrapping_add(mod_page.len() as u32)
                        .wrapping_sub(old_len);
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
                } else {
                    return;
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
                1 => {
                    settings.slot_titles = match settings.slot_titles {
                        SlotTitleMode::Megamix => SlotTitleMode::Original,
                        SlotTitleMode::Original => SlotTitleMode::Internal,
                        SlotTitleMode::Internal | SlotTitleMode::Infernal => SlotTitleMode::Megamix,
                    }
                }
                _ => {}
            },
            MenuAction::UpdateScreen => {}
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
}
