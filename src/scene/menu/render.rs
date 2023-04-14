use ctru::console::Console;

use crate::{
    constants::{
        SLOT_NAMES_DEFAULT, SLOT_NAMES_GATE, SLOT_NAMES_INFERNAL_GATE, SLOT_NAMES_INTERNAL,
        SLOT_NAMES_INTERNAL_GATE, SLOT_NAMES_NORETCON,
    },
    format::barista_cfg::{BaristaConfig, SlotTitleMode},
    launcher::GameVer,
};

use super::{MenuState, SubMenu};

impl MenuState {
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
                #[cfg(feature = "audio")]
                println!(" [{}] Music", if self.cursor == 2 { "*" } else { " " });

                let cursor_increase = if cfg!(feature = "audio") { 1 } else { 0 };

                println!(
                    " [{}] Settings",
                    if self.cursor == 2 + cursor_increase {
                        "*"
                    } else {
                        " "
                    }
                );
                println!(
                    " [{}] Exit Barista",
                    if self.cursor == 3 + cursor_increase {
                        "*"
                    } else {
                        " "
                    }
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
            SubMenu::SetUp(c) => {
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
                    println!("Press A to enable or disable mods");
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
                            // TODO: slot mode
                            if !*c || elmt.1 == u16::MAX {
                                elmt.0.clone()
                            } else if elmt.1 >= 0x100 {
                                String::from("->")
                                    + *match settings.slot_titles {
                                        SlotTitleMode::Internal => SLOT_NAMES_INTERNAL_GATE,
                                        SlotTitleMode::Megamix | SlotTitleMode::Original => {
                                            SLOT_NAMES_GATE
                                        }
                                        SlotTitleMode::Infernal => SLOT_NAMES_INFERNAL_GATE,
                                    }
                                    .get((elmt.1 - 0x100) as usize)
                                    .unwrap_or(&"slot not found")
                            } else {
                                String::from("->")
                                    + *match settings.slot_titles {
                                        // TODO: remove &s when they're all done
                                        SlotTitleMode::Internal => &SLOT_NAMES_INTERNAL,
                                        SlotTitleMode::Megamix => SLOT_NAMES_DEFAULT,
                                        SlotTitleMode::Original => SLOT_NAMES_NORETCON,
                                        SlotTitleMode::Infernal => &["unimplemented uwu"; 0x68],
                                    }
                                    .get(elmt.1 as usize)
                                    .unwrap_or(&"slot not found")
                            }
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
            #[cfg(feature = "audio")]
            SubMenu::Music => {
                println!("Barista - Music");
                println!();
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
                println!(" [{}] Back", if self.cursor == 1 { "*" } else { " " })
            }
            SubMenu::Options => {
                println!("Barista - Settings");
                println!();
                println!(
                    " [{}] Use 0x100 format for gates: {}",
                    if self.cursor == 0 { "*" } else { " " },
                    if settings.original_gates { "on" } else { "off" }
                );
                println!(
                    " [{}] Slot title mode: {}",
                    if self.cursor == 1 { "*" } else { " " },
                    match settings.slot_titles {
                        SlotTitleMode::Megamix => "Megamix",
                        SlotTitleMode::Original => "Original",
                        SlotTitleMode::Internal => "Internal",
                        SlotTitleMode::Infernal => "Infernal...?",
                    }
                );
                println!();
                println!(" [{}] Back", if self.cursor == 2 { "*" } else { " " })
            }
            #[cfg(debug_assertions)]
            SubMenu::Log => {
                println!("{}", unsafe { &crate::log::LOG });
            }
        }
    }
}
