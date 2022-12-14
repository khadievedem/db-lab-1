extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use colored::Colorize;
use eframe::egui::TextBuffer;
use enums::{Keys, Menus};
use std::io::Write;

mod enums;
mod gui;
mod logic;

fn main() {
    let mut generated;
    let mut log = String::new();
    let mut current_key = Keys::MainMenuKey;
    let mut current_menu = Menus::new();

    loop {
        let mut tables: Vec<String> = Vec::new();

        let paths = std::fs::read_dir("../generated_tables/").unwrap();
        for path in paths {
            tables.push(path.unwrap().file_name().into_string().unwrap());
        }

        generated = is_testing_table_exist();

        match current_key {
            Keys::MainMenuKey => {
                current_menu = Menus::Main {
                    is_test_generated: generated,
                    text: vec![
                        "1. Create table.",
                        "2. Delete table.",
                        "3. Edit table.",
                        "4. Table list.",
                        "5. Backup menu",
                        "-------------------------",
                        "8. Generate testing table.",
                        "9. Print testing table.",
                        "-------------------------",
                        "10. Exit (or q).",
                    ],
                };
            }
            Keys::CreateTablKey => current_menu = Menus::Create,
            Keys::DeleteTablKey => {
                current_menu = Menus::Delete {
                    tables_list: tables,
                };
            }
            Keys::EditTablKey => {
                current_menu = Menus::Edit {
                    tables_list: tables,
                };
            }
            Keys::TablListKey => {
                log = format!("{}", get_tables_list(tables));
            }
            Keys::BackupTablKey => {
                current_menu = Menus::Backup {
                    tables_list: tables,
                    backup_path: "../backups/".to_string(),
                };
            }
            Keys::GenTestTablKey => {
                logic::tablgen::gen_test_table();
                log = format!(
                    "{:30}",
                    "TESTING TABLE GENERATED!".bold().green().on_black()
                );
                current_key = Keys::MainMenuKey;
                continue;
            }
            Keys::PrintKey => {
                if generated {
                    logic::tablmgr::create(".temp".to_string(), 'a');
                    let options = eframe::NativeOptions::default();
                    eframe::run_native(
                        "Informational table",
                        options,
                        Box::new(|_cc| Box::new(gui::tablgui::MyApp)),
                    );
                }
            }
            Keys::ExitKey => {
                match logic::tablmgr::clean() {
                    Ok(()) => (),
                    Err(_) => (),
                };
                break;
            }
            _ => {
                log = format!("{:30}", "TRY ONE MORE TIME!".bold().bright_red().on_black());
            }
        };
        current_key = menu_to_show(current_menu.clone(), log.clone());
        log.clear();
    }
}

fn get_tables_list(tables: Vec<String>) -> String {
    let mut log;
    let mut counter = 0;

    if tables.is_empty() {
        log = format!(
            "{:30}",
            "THERE ARE NO TABLES".bold().bright_red().on_black()
        );
    } else {
        log = format!("{:30}", "TABLES LIST:".green().bold().on_black()) + "\n";
        for i in tables {
            log += counter.to_string().as_str();
            log += ". ";
            log += i.as_str();
            log += "\n";
            counter += 1;
        }
    }
    log
}

fn menu_to_show(menus: Menus, log: String) -> Keys {
    print!("{}[2J", 27 as char);
    let mut res = String::new();

    match menus {
        Menus::Main {
            is_test_generated: generated,
            text: text_print,
        } => {
            for i in text_print {
                if i.split(".").nth(0).unwrap().as_str() == "9" {
                    if generated {
                        println!("{}", i);
                    }
                } else {
                    println!("{}", i);
                }
            }
            println!("{}", log);
            print!("> ");
            std::io::stdout().flush().unwrap();

            std::io::stdin().read_line(&mut res).unwrap();
            match res.as_str().replace("\n", "").to_lowercase().as_str() {
                "1" => return Keys::CreateTablKey,
                "2" => return Keys::DeleteTablKey,
                "3" => return Keys::EditTablKey,
                "4" => return Keys::TablListKey,
                "5" => return Keys::BackupTablKey,
                "8" => return Keys::GenTestTablKey,
                "9" => return Keys::PrintKey,
                "10" | "q" | "exit" => return Keys::ExitKey,
                _ => return Keys::UnknownKey,
            }
        }
        Menus::Edit {
            tables_list: tables,
        } => {
            println!("Choose which table do you want to edit?");
            println!("{}", get_tables_list(tables));

            print!("> ");
            std::io::stdout().flush().unwrap();

            std::io::stdin().read_line(&mut res).unwrap();
            // TODO: Edit
        }
        Menus::Create => {
            println!(
                "{:30}",
                "Enter new table's name:".bright_blue().bold().on_black()
            );

            print!("> ");
            std::io::stdout().flush().unwrap();

            std::io::stdin().read_line(&mut res).unwrap();
            let tabl_name = res
                .as_str()
                .replace("\n", "")
                .to_lowercase()
                .as_str()
                .replace(" ", "_");

            logic::tablmgr::create(tabl_name.clone(), 'a');
            println!(
                "{} {} {}",
                "Table".green().on_black(),
                tabl_name.green().bold().on_black(),
                "created successfully!".green().on_black()
            );
            print!("Press any key to continue...");
            std::io::stdout().flush().unwrap();

            std::io::stdin().read_line(&mut res).unwrap();
            return Keys::MainMenuKey;
        }
        Menus::Delete {
            tables_list: tables,
        } => {
            println!(
                "{:30}",
                "Choose number which table do you want to DELETE? (or c for cancel)"
                    .bright_red()
                    .bold()
                    .on_black()
            );
            println!("{}", get_tables_list(tables.clone()));

            print!("> ");
            std::io::stdout().flush().unwrap();

            std::io::stdin().read_line(&mut res).unwrap();

            let delete_option = res.to_lowercase().as_str().replace("\n", "");
            if delete_option.eq("c") {
                println!(
                    "{:30}",
                    "Deleting canceled successfully!".green().bold().on_black()
                );
            } else {
                let index: usize = match delete_option.parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("{:30}", "Error while deleting".red().bold().on_black());
                        return Keys::DeleteTablKey;
                    }
                };
                let del_table_name = tables.get(index).unwrap().to_string();
                match logic::tablmgr::delete(del_table_name.clone()) {
                    Ok(()) => {
                        println!(
                            "{} {} {}",
                            "Deleted".green().on_black(),
                            del_table_name.green().bold().on_black(),
                            "successfully!".green().on_black()
                        )
                    }
                    Err(_) => {
                        println!("{:30}", "Error while deleting".red().bold().on_black());
                        return Keys::DeleteTablKey;
                    }
                }
                print!("Press any key to continue...");
                std::io::stdout().flush().unwrap();

                std::io::stdin().read_line(&mut res).unwrap();
            }
            return Keys::MainMenuKey;
        }
        Menus::Backup {
            tables_list: tables,
            backup_path: path,
        } => {
            println!(
                "{:30}",
                "Do you want to make Backup? (y/n)"
                    .bright_yellow()
                    .bold()
                    .on_black()
            );

            print!("> ");
            std::io::stdout().flush().unwrap();

            std::io::stdin().read_line(&mut res).unwrap();

            match res.as_str().replace("\n", "").to_lowercase().as_str() {
                "y" | "yes" => {
                    let timenow: DateTime<Utc> = std::time::SystemTime::now().into();
                    for i in tables {
                        let time = timenow.format("%d-%m-%Y-%H:%M");
                        std::fs::copy(
                            format!("../generated_tables/{}", i),
                            format!("{}{}-{}", path, time, i),
                        )
                        .unwrap();
                    }

                    println!("{}", "Backup successfully done!".green().on_black());

                    print!("Press any key to continue...");
                    std::io::stdout().flush().unwrap();

                    std::io::stdin().read_line(&mut res).unwrap();
                }
                "n" | "no" => {
                    return Keys::MainMenuKey;
                }
                _ => return Keys::BackupTablKey,
            }
        }
        Menus::Unknown => {
            return Keys::UnknownKey;
        }
    }

    Keys::MainMenuKey
}

fn is_testing_table_exist() -> bool {
    match std::fs::File::open(std::path::Path::new(
        "../generated_tables/testing_table.txt",
    )) {
        Ok(_) => return true,
        Err(_) => return false,
    };
}
