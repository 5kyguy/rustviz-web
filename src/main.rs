// rust lib
use std::{collections::BTreeMap, env, path::Path};

use rustviz_lib::data::VisualizationData;
use rustviz_lib::parse;
use rustviz_lib::svg_frontend::svg_generation;

fn main() {
    // verify usage
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage Error: cargo run <filename>");
        return;
    }

    let path_to_ex = Path::new("examples").join(&args[1]);
    if !path_to_ex.is_dir() {
        println!("Error: no corresponding directory exists in examples/!");
        return;
    }

    let filename = path_to_ex.join("main.rs");
    if !Path::new(&filename).is_file() {
        println!("Error: Example source file (main.rs) not found in {:?}!", &filename);
        return;
    }

    println!("{:?}", filename);
    let main_src = match std::fs::read_to_string(&filename) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let (rest, line_num, var_map) = match parse::parse_vars_to_map_str(&main_src) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let events = match parse::extract_events(&rest, line_num) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
        preprocess_external_events: Vec::new(),
        event_line_map: BTreeMap::new(),
    };

    if let Err(e) = parse::add_events(&mut vd, var_map, events) {
        eprintln!("{}", e);
        return;
    }

    let input_path = path_to_ex
        .join("input/")
        .into_os_string()
        .into_string()
        .expect("Error in input file path!");
    let output_path = path_to_ex
        .into_os_string()
        .into_string()
        .expect("Error in output file path!");
    svg_generation::render_svg(&input_path, &(output_path + "/"), &mut vd);
}
