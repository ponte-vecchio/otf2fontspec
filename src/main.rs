mod cli;
mod util;

fn main() {
    let cli_matches = cli::cli().get_matches();
    match cli_matches.subcommand() {
        Some(("list", list_matches)) => {
            // get all flags
            let (all, sup, unsup, dep) = (
                list_matches.get_flag("all"),
                list_matches.get_flag("supported"),
                list_matches.get_flag("unsupported"),
                list_matches.get_flag("deprecated")
            );
            // check boolean flag for "all"
            if all { util::print_all_features(); }
            // if any one of sup, unsup, dep is true
            else if sup || unsup || dep {
                util::print_selected_features(sup, unsup, dep);
            }
            else {
                panic!("Not implemented yet")
            }
        }
        Some(("tag", tag_matches)) => {
            let tag_name = tag_matches
                .get_one::<String>("feature")
                .expect("No feature name provided");
            util::print_one_detailed(tag_name.to_string());
        }
        _ => unreachable!(),
    }
}
