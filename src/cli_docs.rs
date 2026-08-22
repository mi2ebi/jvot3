use std::{borrow::Borrow as _, sync::LazyLock};

use flower_pot::{
    BOLD, BRIGHT_BLUE as BLUE, BRIGHT_CYAN as CYAN, BRIGHT_YELLOW as YELLOW, ITALIC,
    /* NORMAL_INTENSITY as REGULAR, */ RESET,
};

fn color(c: &str, s: &str) -> String { color_then(c, s, RESET) }
fn color_then(c: &str, s: &str, t: &str) -> String { format!("{c}{s}{t}") }

pub fn lit(s: &str) -> String { lit_then(s, RESET) }
pub fn lit_then(s: &str, t: &str) -> String { format!("{BOLD}{BLUE}{s}{t}") }

pub fn var(s: &str) -> String { var_then(s, RESET) }
pub fn var_then(s: &str, t: &str) -> String { format!("{BOLD}{ITALIC}{CYAN}{s}{t}") }

fn ques(s: &str) -> String { format!("{s}?") }

const VERSION: &str = "(3.0.0 dev)";

const SP: &str = " ";
const NL: &str = "\n";

macro_rules! cat {
    [$($e:expr),* $(,)?] => {{
        let mut _s = String::new();
        $(_s.push_str($e.borrow());)*
        _s
    }}
}

pub static VERSTR: LazyLock<String> =
    LazyLock::new(|| cat![YELLOW, BOLD, "jvot3 ", VERSION, RESET]);

pub static TUI_DOCS: LazyLock<String> = LazyLock::new(|| {
    cat![
        VERSTR,
        SP,
        color(CYAN, "interactive"),
        NL,
        "at each prompt you may enter non-empty strings of the form",
        NL,
        ques(&cat![lit(":"), var("function")]),
        SP,
        ques(&var("input")),
        NL.repeat(2),
        "functions",
        NL,
        lit(":h"),
        " - show this help",
        NL,
        lit(":q"),
        " - quit",
        NL,
        lit(":units"),
        SP,
        var("text"),
        " - split ",
        var("text"),
        " into units"
    ]
});

pub static NO_FN_HINT: LazyLock<String> = LazyLock::new(|| {
    cat![
        YELLOW,
        "what should i do with this? the functions i have are described in ",
        lit(":h"),
        YELLOW,
        ", and specifying one is mandatory for now",
        RESET
    ]
});

pub static UNKNOWN_FN_HINT: LazyLock<String> = LazyLock::new(|| {
    cat![YELLOW, "idk how to do that. the functions i have are described in ", lit(":h")]
});

pub static EMPTY_INPUT_HINT: LazyLock<String> =
    LazyLock::new(|| cat![YELLOW, "\"\" to you too!", RESET]);

pub static BYE: LazyLock<String> = LazyLock::new(|| cat![RESET, "bye <3"]);
