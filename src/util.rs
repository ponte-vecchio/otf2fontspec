use colored::{self, Colorize};
use opentype;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[cfg(target_os = "macos")]
const LOCFONTDIR: &str = "Library/Fonts";
const SYSFONTDIR: &str = "/System/Library/Fonts";

#[derive(Debug)]
struct FontMatch {
    matched: bool,
    location: String,
}

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
    let mut tag_names: Vec<String> = hashmap.keys().cloned().collect();
    tag_names.sort();
    (tag_names, hashmap)
}

/// Prints the header of the table.
pub fn print_header(print_for: &str) {
    let header_1 = "Feature";
    let header_2 = "Description";
    let header_3 = match print_for {
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

pub fn fprint(row1: &str, row2: &str, row3: &str) {
    println!("{:<9}{:<40}{}", row1.bold().cyan(), row2, row3.magenta());
}

/// Prints all OTF tags, names and fontspec options.
pub fn print_all_features() {
    let (mut tag_names, otf_hashmap) = get_otf_names_and_features();
    /* print header */
    print_header("");
    // make a sorted array from tag_names
    tag_names.sort();

    for f in tag_names.iter() {
        let desc_brief = otf_hashmap[f]["desc"][0].as_str().unwrap();
        // let desc_long = otf_hashmap[f]["desc"][1].as_str().unwrap();
        let fontspec_cmd = otf_hashmap[f]["cmd"].as_str().unwrap();

        // do not print fontspec_cmd if it's equal to "None"
        if fontspec_cmd == "None" {
            fprint(f, desc_brief, "-");
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

    let (tag_names, otfdict) = get_otf_names_and_features();

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
        let mut keys_to_use = tags_per_type[mode].clone();
        keys_to_use.sort();
        print_header(mode.replace("\"", "").as_str());
        for v in keys_to_use.iter() {
            let desc_brief = otfdict[v]["desc"][0].as_str().unwrap();
            fprint(
                v,
                desc_brief,
                mode.replace("\"", "")
                    .replace("-", " ")
                    .to_uppercase()
                    .as_str(),
            );
        }
    } else {
        print_header("");
        for k in tag_names.iter() {
            if otfdict[k]["type"] == "not-supported" || otfdict[k]["type"] == "deprecated" {
                continue;
            } else {
                let desc_brief = otfdict[k]["desc"][0].as_str().unwrap();
                let fontspec_cmd = otfdict[k]["cmd"].as_str().unwrap();
                fprint(k, desc_brief, fontspec_cmd);
            }
        }
    }
}

/// Prints a single OTF tag, name and fontspec option and detailed description.
pub fn print_one_detailed(tag_name: &str) {
    let mut tag_name = tag_name.to_lowercase().replace(" ", "");
    let (feature_tags, otf_hashmap) = get_otf_names_and_features();
    let mut fontspec_left = "\n\\setmainfont{...}[";
    let mut fontspec_right = "]\n\n";

    // exit early if tag_name is not found in feature_tags
    if !feature_tags.contains(&tag_name) {
        println!("Feature name not found.");
        std::process::exit(1);
    } else {
        // TODO: print suggestions for the closest match from feature_tags
        //       e.g. c2dc -> "Did you mean c2sc?"
        let desc_brief = otf_hashmap[&tag_name]["desc"][0].as_str().unwrap();
        let desc_long = otf_hashmap[&tag_name]["desc"][1].as_str().unwrap();
        let mut fontspec_cmd = otf_hashmap[&tag_name]["cmd"].as_str().unwrap();
        tag_name = tag_name.to_uppercase();
        // check if desc_brief is "None", and make fontspec_* as empty string.
        if fontspec_cmd == "None" {
            (fontspec_left, fontspec_cmd, fontspec_right) = ("", "\n", "");
        }
        println!(
            "\n{:<8}{}\n{}{}{}{}",
            tag_name, desc_brief, fontspec_left, fontspec_cmd, fontspec_right, desc_long
        );
    }
}

/// Get OTF features for a given font.
pub fn font_finder(fontname: &str, fontdir: &str) {
    let re = format!(
        r"{}([-\s]?[rR](egular|oman))?.otf$",
        &fontname.replace(" ", "").replace("-", "")
    );
    let mut is_default_dir = false;
    let mut query_result = FontMatch {
        matched: false,
        location: String::new(),
    };

    // check if fontdir is empty
    if fontdir.is_empty() {
        is_default_dir = true;
    }

    if !is_default_dir {
        let lookdir = Path::new(fontdir);
        for f in fs::read_dir(&lookdir).unwrap() {
            let file = f.unwrap();
            let filename = file.file_name().into_string().unwrap();
            println!("Looking for {} in {}", re, filename);
            if Regex::new(&re).unwrap().is_match(&filename) {
                println!("matched");
            }
        }
    } else {
        // use LOCFONTDIR and SYSFONTDIR
        let lookdirs = vec![
            Path::new(&env::var("HOME").unwrap()).join(LOCFONTDIR),
            Path::new(SYSFONTDIR).to_path_buf(),
        ];

        for ld in lookdirs {
            println!("Looking in {:?}", ld);
            for f in fs::read_dir(&ld).unwrap() {
                let file = f.unwrap();
                let filename = file.file_name().into_string().unwrap();
                // println!("Looking for {} in {}", re, filename);
                if Regex::new(&re).unwrap().is_match(&filename) {
                    query_result.matched = true;
                    query_result.location = file.path().to_str().unwrap().to_string();
                    println!("{:?}", query_result);
                }
            }
        }
        // for f in fs::read_dir(&Path::new(&env::var("HOME").unwrap()).join(LOCFONTDIR)).unwrap() {
        //     let file = f.unwrap();
        //     let filename = file.file_name().into_string().unwrap();
        //     println!("Looking for {} in {}", re, filename);
        //     if Regex::new(&re).unwrap().is_match(&filename) {
        //         println!("matched");
        //     }
        // }
    }
}
