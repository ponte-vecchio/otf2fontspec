use clap::{Arg, ArgAction, Command};
/// Ideas on command line options
// -a, --all: print all features and commands
// -t, --feature: print fontspec command for a given OTF feature
// -q, --query: query a font, find its OTF features and print their fontspec commands
// -d, --directory: optional if argument for -q exists. Searches for a given font in that directory
// -h, --help: print help message
// -v, --version: print version number
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}
macro_rules! crate_author {
    () => {
        env!("CARGO_PKG_AUTHORS")
    };
}

pub fn cli() -> Command {
    let cmd = Command::new("otf2fontspec")
        .about("CLI tool to convert OpenType features to fontspec commands")    
        .author(crate_author!())
        .bin_name("otf2fontspec")
        .version(crate_version!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            // Show all
            Command::new("list")
                .short_flag('l')
                .long_flag("list")
                .about("Print all features and their fontspec commands, if available")
                .arg(/* -la */
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .help("Print all features, including unsupported and deprecated ones")
                        .conflicts_with_all(["supported","deprecated", "unsupported"])
                        .num_args(0)
                        .action(ArgAction::SetTrue)
                )
                .arg(/* -ld */
                    Arg::new("deprecated")
                        .short('d')
                        .long("deprecated")
                        .help("Show deprecated OTF tags")
                        .conflicts_with_all(["all", "supported", "unsupported"])
                        .num_args(0)
                        .action(ArgAction::SetTrue)
                )
                .arg(/* -ls */
                    Arg::new("supported")
                        .short('s')
                        .long("supported")
                        .help("Show OTF tags supported by fontspec")
                        .conflicts_with_all(["all", "deprecated", "unsupported"])
                        .num_args(0)
                        .action(ArgAction::SetTrue)
                )
                .arg(/* -lu */
                    Arg::new("unsupported")
                        .short('u')
                        .long("unsupported")
                        .help("Show OTF tags not supported by fontspec")
                        .conflicts_with_all(["all", "supported", "deprecated"])
                        .num_args(0)
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            // Tag subcommand
            Command::new("tag")
                .short_flag('t')
                .long_flag("tag")
                .about("Print fontspec command for a given OTF feature tag")
                .arg(
                    Arg::new("feature")
                        .help("OTF feature to print fontspec command for")
                        .required(true)
                        .index(1)
                        .action(ArgAction::Set)
                )
        );
        cmd
}