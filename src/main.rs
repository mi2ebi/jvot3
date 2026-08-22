use std::io::{Write as _, stdout};

use flower_pot::{BOLD, BRIGHT_CYAN as CYAN, BRIGHT_RED as RED, RESET};
use jvot3::{
    settings::{
        HyphenSetting::{AllowY, ForceY, Standard},
        Settings,
    },
    units::unitify,
};
use rustyline::{DefaultEditor, error::ReadlineError};

mod cli_docs;
use crate::cli_docs::{BYE, EMPTY_INPUT_HINT, NO_FN_HINT, TUI_DOCS, UNKNOWN_FN_HINT};

fn settings_label(settings: Settings) -> String {
    let mut parts = vec![];
    // todo phonology
    // todo rafsi
    match settings.hyphens {
        Standard => {}
        AllowY => parts.push("allow-y"),
        ForceY => parts.push("force-y"),
    }
    if settings.generate_cmevla {
        parts.push("generate-cmevla");
    }
    if settings.arbitrary_cmavo_rafsi {
        parts.push("arbitrary-cmavo-rafsi");
    }
    if settings.allow_mz {
        parts.push("allow-mz");
    }
    if settings.no_slinkuhi {
        parts.push("no-slinku'i");
    }
    parts.join(" ")
}

fn build_prompt(settings: Settings) -> String {
    let label = settings_label(settings);
    if label.is_empty() {
        format!("\n{RESET}{CYAN}>{RESET} {BOLD}")
    } else {
        format!("\n{RESET}{CYAN}[{label}]{RESET}\n{CYAN}>{RESET} {BOLD}")
    }
}

fn main() {
    let mut rl = DefaultEditor::new().expect("failed to create line editor");
    let settings = Settings::CLL;
    loop {
        let prompt = build_prompt(settings);
        match rl.readline(&prompt) {
            Ok(input) => {
                print!("{RESET}");
                stdout().flush().unwrap();
                rl.add_history_entry(input.as_str()).ok();
                let rest = input.trim();
                if rest.is_empty() {
                    println!("{}", *EMPTY_INPUT_HINT);
                    continue;
                }
                let Some(after_colon) = rest.strip_prefix(':') else {
                    println!("{}", *NO_FN_HINT);
                    continue;
                };
                let (fun, arg) =
                    after_colon.split_once(char::is_whitespace).unwrap_or((after_colon, ""));
                let arg = arg.trim();
                match fun {
                    "q" => {
                        println!("{}", *BYE);
                        break;
                    }
                    "h" => println!("{}", *TUI_DOCS),
                    "units" => match unitify(arg, settings) {
                        Ok(us) => {
                            print!("{}: ", us.len());
                            for u in us {
                                print!("{u:?} ");
                            }
                            println!();
                        }
                        Err(e) => println!("{RED}{e}{RESET}"),
                    },
                    _ => println!("{}", *UNKNOWN_FN_HINT),
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!("{}", *BYE);
                break;
            }
            Err(e) => {
                println!("{RESET}{RED}readline error: {e}{RESET}");
                break;
            }
        }
    }
}
