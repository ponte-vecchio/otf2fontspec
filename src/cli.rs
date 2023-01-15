use clap::{Arg, ArgAction, Command};

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
                .arg(
                    /* -la */
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .help("Print all features, including unsupported and deprecated ones")
                        .conflicts_with_all(["supported", "deprecated", "unsupported"])
                        .num_args(0)
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    /* -ld */
                    Arg::new("deprecated")
                        .short('d')
                        .long("deprecated")
                        .help("Show deprecated OTF tags")
                        .conflicts_with_all(["all", "supported", "unsupported"])
                        .num_args(0)
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    /* -ls */
                    Arg::new("supported")
                        .short('s')
                        .long("supported")
                        .help("Show OTF tags supported by fontspec")
                        .conflicts_with_all(["all", "deprecated", "unsupported"])
                        .num_args(0)
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    /* -lu */
                    Arg::new("unsupported")
                        .short('u')
                        .long("unsupported")
                        .help("Show OTF tags not supported by fontspec")
                        .conflicts_with_all(["all", "supported", "deprecated"])
                        .num_args(0)
                        .action(ArgAction::SetTrue),
                ),
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
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            // Query subcommand
            Command::new("query")
                .short_flag('q')
                .long_flag("query")
                .about("Query a font, find its OTF features and print their fontspec commands")
                .arg(
                    /* -q <font> */
                    Arg::new("font")
                        .help("Font to query")
                        .required(true)
                        .index(1)
                        .action(ArgAction::Set),
                )
                .arg(
                    /* -qd <font> <directory> */
                    Arg::new("directory")
                        .short('d')
                        .long("directory")
                        .help("Directory to search for font in")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    /* -qt <font> <tag> */
                    /* TODO: Should account for -qdt/-qtd */
                    Arg::new("tag")
                        .short('t')
                        .long("tag")
                        .help("OTF feature tag to find for a given font")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    /* -qa <font> */
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .help("Show all OTF features for a given font")
                        .required(false)
                        .num_args(0)
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["supported", "unsupported", "tag"]),
                )
                .arg(
                    /* -qs <font> */
                    Arg::new("supported")
                        .short('s')
                        .long("show")
                        .help("Show OTF tags supported by fontspec for a given font")
                        .required(false)
                        .num_args(0)
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["all", "unsupported", "tag"]),
                )
                .arg(
                    /* -qu <font> */
                    Arg::new("unsupported")
                        .short('u')
                        .long("unsupported")
                        .help("Show OTF tags not supported by fontspec for a given font")
                        .required(false)
                        .num_args(0)
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["all", "supported", "tag"]),
                ),
        );
    cmd
}
