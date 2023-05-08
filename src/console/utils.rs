pub fn welcome_lines() -> Vec<String> {
    let lines: Vec<&str> = vec![
        "// -------------------------------------------------------------------------- //",
        "//                                                                            //",
        "//                              BOZOS ver 3.0.2                               //",
        "//                                                                            //",
        "//                          Compute with confidence.                          //",
        "// -------------------------------------------------------------------------- //",
        " ",
        "+ SYSTEM STATE --------------------------------------------------------------  +",
        "> System up to date and operational",
        " ",
        "+ HOW TO USE ----------------------------------------------------------------  +",
        "To interact with this terminal, type commands in the input line below.",
        "Then, execute the command using the <Return> key on your (real) keyboard.",
        "If you are lost, enter 'help' to show this message again.",
        " ",
        "+ AVAILABLE COMMANDS --------------------------------------------------------  +",
        "    - help: show the available commands",
        "    - cheat <code>: enable a cheat code to activate an ability",
        "    - log: display a log entry",
        "    - clear: clear the entire display",
        "    - exit: exit this terminal to go back to boring reality",
        " ",
        " ",
    ];

    lines
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}
