use colored::{self, Colorize};
use serde_json::Value;
use std::collections::HashMap;

// pub fn get_otf_features() -> HashMap<String, Value> {
//     let otf_hashmap: HashMap<String, Value> =
//         serde_json::from_str(include_str!("map.json")).unwrap();
//     otf_hashmap
// }

// pub fn get_otf_tag_names(hash_map: Option<HashMap<String, Value>>) -> Vec<String> {
//     // hash_map can be HashMap or None
//     // if None, get_otf_features() is called
//     let otf = match hash_map {
//         Some(hash_map) => hash_map,
//         None => get_otf_features(),
//     };
//     let tag_names: Vec<_> = otf.keys().cloned().collect();
//     tag_names
// }

/// Returns a parsed hashmap and the tag names as a tuple.
pub fn get_otf_names_and_features() -> (Vec<String>, HashMap<String, Value>) {
    let hashmap: HashMap<String, Value> = serde_json::from_str(include_str!("map.json")).unwrap();
    let mut tag_names: Vec<_> = hashmap.keys().cloned().collect();
    tag_names.sort();
    (tag_names, hashmap)
}

/// Prints the header of the table.
pub fn print_header(print_for: String) {
    let header_1 = "Feature";
    let header_2 = "Description";
    let header_3 = match print_for.as_str() {
        "deprecated" | "unsupported" => "",
        print_for if print_for.contains("-") => "",
        _ => "Fontspec Option",
    };
    /* format it so that header_1 is left-aligned in 10 spaces,
     * header_2 is left_aligned in 24 spaces,
     * and the final header_3 is left-aligned with no space restriction
     */
    println!(
        "{:<9}{:<40}{}",
        header_1.bold().yellow(),
        header_2.bold().yellow(),
        header_3.bold().yellow()
    );
}

pub fn fprint(
    row1: &str,
    row2: &str,
    row3: &str
) {
    println!("{:<9}{:<40}{}", row1.bold().cyan(), row2, row3.magenta());
}

/// Prints all OTF tags, names and fontspec options.
pub fn print_all_features() {
    let (mut tag_names, otf_hashmap) = get_otf_names_and_features();
    /* print header */
    print_header("".to_string());
    // make a sorted array from tag_names
    tag_names.sort();

    for f in tag_names.iter() {
        let desc_brief = otf_hashmap[f]["desc"][0].as_str().unwrap();
        // let desc_long = otf_hashmap[f]["desc"][1].as_str().unwrap();
        let fontspec_cmd = otf_hashmap[f]["cmd"].as_str().unwrap();

        // do not print fontspec_cmd if it's equal to "None"
        if fontspec_cmd == "None" {
            fprint(
                f, desc_brief, "-");
        } else {
            fprint(f, desc_brief, fontspec_cmd);
        }
    }
}

/// Prints selected OTF tags, names and fontspec options (if available).
pub fn print_selected_features(supported: bool, unsupported: bool, deprecated: bool) {
    let mode = match (supported, unsupported, deprecated) {
        (true, false, false) => "",
        (false, true, false) => "\"not-supported\"",
        (false, false, true) => "\"deprecated\"",
        _ => unreachable!(),
    };

    let (_, otfdict) = get_otf_names_and_features();

    // extract the values of "type" key
    fn gather_keys_by_value(
        dict: HashMap<String, Value>,
        value: &str,
    ) -> HashMap<String, Vec<String>> {
        let mut gathered_dict: HashMap<String, Vec<String>> = HashMap::new();
        let mut keys_to_check: Vec<_> = dict.keys().cloned().collect();
        keys_to_check.sort(); // needed for our purposes

        // make a hashmap where the key is the "type",
        // and value is an array of keys from the dict that have the same "type" value
        for key in keys_to_check.iter() {
            // let mut otfdict = get_otf_features();
            let type_val = dict[key][&value].to_string();
            if gathered_dict.contains_key(&type_val) {
                // add to the vec if the key exists (also implies the vec exists)
                if let Some(vals) = gathered_dict.get_mut(&type_val) {
                    vals.push(key.to_string().replace("\"", ""));
                }
            } else {
                gathered_dict.insert(
                    type_val.to_string(),
                    vec![key.to_string().replace("\"", "")],
                );
            }
        }
        gathered_dict
    }
    let tags_per_type = gather_keys_by_value(otfdict.clone(), "type");

    if !supported {
        print_header(mode.to_string().replace("\"", ""));
        for v in tags_per_type[mode].iter() {
            let desc_brief = otfdict[v]["desc"][0].as_str().unwrap();
            fprint(v, desc_brief, "not supported");
            // println!(
                // "{:<9}{:<40}{}",
                // v.bold().cyan(),
                // desc_brief,
                // "not supported".red()
            // );
        }
    } else {
        print_header("".to_string());
        for (k, v) in otfdict.iter() {
            if v["type"] == "not-supported" || v["type"] == "deprecated" {
                continue;
            } else {
                let desc_brief = v["desc"][0].as_str().unwrap();
                let fontspec_cmd = v["cmd"].as_str().unwrap();
                fprint(k, desc_brief, fontspec_cmd);
            }
        }
    }
}

/// Prints a single OTF tag, name and fontspec option and detailed description.
pub fn print_one_detailed(tag_name: String) {
    let mut tag_name = tag_name.to_lowercase().replace(" ", "");
    let (feature_tags, otf_hashmap) = get_otf_names_and_features();
    let mut fontspec_left: String = "\n\\setmainfont{...}[".to_string();
    let mut fontspec_right: String = "]\n\n".to_string();

    // exit early if tag_name is not found in feature_tags
    // TODO: print suggestions for the closest match from feature_tags
    //       e.g. "Did you mean c2sc?"
    if !feature_tags.contains(&tag_name) {
        println!("Feature name not found.");
        std::process::exit(1);
    } else {
        let desc_brief: String = otf_hashmap[&tag_name]["desc"][0]
            .as_str()
            .unwrap()
            .to_string();
        let desc_long: String = otf_hashmap[&tag_name]["desc"][1]
            .as_str()
            .unwrap()
            .to_string();
        let mut fontspec_cmd: String = otf_hashmap[&tag_name]["cmd"].as_str().unwrap().to_string();
        tag_name = tag_name.to_uppercase();
        // check if desc_brief is "None", and make fontspec_* as empty string.
        if fontspec_cmd == "None" {
            fontspec_left = "".to_string();
            fontspec_right = "\n".to_string();
            fontspec_cmd = "".to_string();
        }
        println!(
            "\n{:<8}{}\n{}{}{}{}",
            tag_name, desc_brief, fontspec_left, fontspec_cmd, fontspec_right, desc_long
        );
    }
}
