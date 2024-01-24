use ctru::{console::Console, services::ps::Ps};

use crate::{
    constants::{
        SLOT_NAMES_DEFAULT, SLOT_NAMES_GATE, SLOT_NAMES_INFERNAL, SLOT_NAMES_INFERNAL_GATE,
        SLOT_NAMES_INTERNAL, SLOT_NAMES_INTERNAL_GATE, SLOT_NAMES_NORETCON,
    },
    format::barista_cfg::{BaristaConfig, SlotTitleMode},
    launcher::GameVer,
    Result,
};

use super::{MenuState, SubMenu};

impl MenuState {
    pub fn render(
        &mut self,
        console: &Console,
        versions: &[GameVer],
        mods: &[(String, u16)],
        page: usize,
        num_pages: usize,
        settings: &BaristaConfig,
    ) -> Result<()> {
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
                    " [{}] Credits",
                    if self.cursor == 3 + cursor_increase {
                        "*"
                    } else {
                        " "
                    }
                );
                println!(
                    " [{}] Exit Barista",
                    if self.cursor == 4 + cursor_increase {
                        "*"
                    } else {
                        " "
                    }
                );
                #[cfg(debug_assertions)]
                println!("\x1b[29;0HBarista debug commit {}", env!("GIT_HASH"));
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
                    println!("A to enable/disable mods");
                    println!("DPad Left/Right to change index");
                    println!("Hold X to scroll indexes faster");
                    println!("L/R change page, Y shows slot names");
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
                                let letters;
                                String::from("->")
                                    + if settings.slot_titles == SlotTitleMode::Infernal
                                        && elmt.1 == 0x58
                                    {
                                        letters = generate_random_letters::<10>()?;
                                        &letters
                                    } else {
                                        *match settings.slot_titles {
                                            SlotTitleMode::Internal => SLOT_NAMES_INTERNAL,
                                            SlotTitleMode::Megamix => SLOT_NAMES_DEFAULT,
                                            SlotTitleMode::Original => SLOT_NAMES_NORETCON,
                                            SlotTitleMode::Infernal => SLOT_NAMES_INFERNAL,
                                        }
                                        .get(elmt.1 as usize)
                                        .unwrap_or(&"slot not found")
                                    }
                            }
                        );
                    }
                    println!();
                    println!(
                        "- [{}] Previous page",
                        if self.cursor == self.cursor_option_len(versions, mods) - 3 {
                            "*"
                        } else {
                            " "
                        }
                    );
                    println!(
                        "- [{}] Next page",
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
                println!(
                    " [{}] Display mod loaded msg: {}",
                    if self.cursor == 2 { "*" } else { " " },
                    if settings.btk_loaded_msg { "on" } else { "off" }
                );
                println!(
                    " [{}] (WIP) Enable extra rows: {}",
                    if self.cursor == 3 { "*" } else { " " },
                    if settings.extra_rows { "on" } else { "off" }
                );
                println!();
                println!(" [{}] Back", if self.cursor == 4 { "*" } else { " " })
            }
            SubMenu::Credits => {
                println!("Barista + Saltwater - Credits:");
                println!();
                println!("Project lead:");
                println!("    patataofcourse");
                println!();
                println!("Art:");
                println!("    MilaTheArtsy");
                println!();
                println!("Programming, RE & research:");
                println!("    patataofcourse");
                println!("    0xAdk");
                println!("    EstexNT");
                println!();
                println!("Alpha testing:");
                println!("    Kievit");
                println!("    Nate Candles");
                println!("    somethingAccurate");
                println!("    TheAlternateDoctor");
                println!("    Tox (tox2564)");
                println!("    Zeo (thatzeogal)");
                println!();
                println!("Special thanks to:");
                println!("    PabloMK7 & Nanquitas");
                println!("    The CTPG-7 Team");
                println!("    The Rust 3DS project");
                println!("    The RHModding community");
                println!();
                println!("Press B to return to the main menu");
            }
            #[cfg(debug_assertions)]
            SubMenu::Log => {
                println!("{}", crate::log::LOG.lock().unwrap());
            }
        }
        Ok(())
    }
}

//TODO: this is pretty ineffective
pub fn generate_random_letters<const I: usize>() -> crate::Result<String> {
    let ps = Ps::new()?;
    let mut bytes = [0; I];
    let mut loop_counter = 0;
    'a: loop {
        loop_counter += 1;
        if loop_counter == 0x10 {
            return Ok("GJI·O&(GN%·=ÊGFMA%D)FJGB^PKGË"[..I].to_string());
        }
        ps.generate_random_bytes(&mut bytes)?;
        for b in bytes {
            //TODO: any other?
            if !(0x20..=0x7f).contains(&b) {
                continue 'a;
            }
        }
        break;
    }
    log!(General, "{:X?}", bytes);
    Ok(String::from_utf8(bytes.to_vec()).unwrap())
}
