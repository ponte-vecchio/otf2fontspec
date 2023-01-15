mod cli;
mod util;

fn main() {
    let cli_matches = cli::cli().get_matches();
    match cli_matches.subcommand() {
        /* Granular actions for list subcommands */
        Some(("list", list_matches)) => {
            // get all flags
            let (all, sup, unsup, dep) = (
                list_matches.get_flag("all"),
                list_matches.get_flag("supported"),
                list_matches.get_flag("unsupported"),
                list_matches.get_flag("deprecated"),
            );
            // check boolean flag for "all"
            if all {
                util::print_all_features();
            }
            // if any one of sup, unsup, dep is true
            else if sup || unsup || dep {
                util::print_selected_features(sup, unsup, dep);
            } else {
                // default to supported (= "-ls")
                util::print_selected_features(true, false, false);
            }
        }
        /* Granular actions for tag subcommands if any */
        Some(("tag", tag_matches)) => {
            let tag_name = tag_matches
                .get_one::<String>("feature")
                .expect("No feature name provided");

            // arg must be a 4-letter alphanumeric string
            if !tag_name.chars().all(char::is_alphanumeric) || tag_name.len() != 4 {
                eprintln!("{}: Invalid feature tag name", tag_name);
                std::process::exit(1);
            }

            util::print_one_detailed(tag_name);
        }
        Some(("query", font_matches)) => {
            util::font_finder(
                font_matches
                    .get_one::<String>("font")
                    .expect("No font name provided"),
                font_matches
                    .get_one::<String>("directory")
                    .get_or_insert(&String::new()),
            );
        }
        _ => unreachable!(),
    }
}
