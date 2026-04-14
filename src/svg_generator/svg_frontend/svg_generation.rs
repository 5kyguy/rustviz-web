extern crate handlebars;

use crate::data::{ResourceAccessPoint_extract, Visualizable, VisualizationData, LINE_SPACE};
use crate::error::RustvizError;
use crate::svg_frontend::{code_panel, timeline_panel, utils};
use handlebars::Handlebars;
use serde::Serialize;
use std::cmp;

#[derive(Serialize)]
struct SvgData {
    visualization_name: String,
    css: String,
    code: String,
    diagram: String,
    tl_id: String,
    tl_width: i32,
    height: i32,
}

const CODE_TEMPLATE: &str = include_str!("../templates/code_template.svg");
const TIMELINE_TEMPLATE: &str = include_str!("../templates/timeline_template.svg");
const BOOK_SVG_STYLE: &str = include_str!("../templates/book_svg_style.css");

/// Render SVGs from annotated source, plain source for layout, and visualization state.
pub fn render_svg_strings(
    annotated_source: &str,
    source_plain: &str,
    viz_name: &str,
    visualization_data: &mut VisualizationData,
) -> Result<(String, String), RustvizError> {
    preprocess_viz_data(visualization_data);

    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(handlebars::no_escape);
    handlebars
        .register_template_string("code_svg_template", CODE_TEMPLATE)
        .map_err(|e| RustvizError::Parse(format!("template: {}", e)))?;
    handlebars
        .register_template_string("timeline_svg_template", TIMELINE_TEMPLATE)
        .map_err(|e| RustvizError::Parse(format!("template: {}", e)))?;

    let mut max_x_space: i64 = 0;
    let (code_panel_string, num_lines) = code_panel::render_code_panel(
        annotated_source,
        source_plain,
        &mut max_x_space,
        &visualization_data.event_line_map,
    );

    let (timeline_panel_string, max_width) = timeline_panel::render_timeline_panel(visualization_data);

    let svg_data = SvgData {
        visualization_name: viz_name.to_string(),
        css: BOOK_SVG_STYLE.to_string(),
        code: code_panel_string,
        diagram: timeline_panel_string,
        tl_id: "tl_".to_owned() + viz_name,
        tl_width: cmp::max(max_width, 200),
        height: (num_lines * LINE_SPACE as i32 + 80) + 50,
    };

    let final_code_svg_content = handlebars
        .render("code_svg_template", &svg_data)
        .map_err(|e| RustvizError::Parse(format!("render code svg: {}", e)))?;
    let final_timeline_svg_content = handlebars
        .render("timeline_svg_template", &svg_data)
        .map_err(|e| RustvizError::Parse(format!("render timeline svg: {}", e)))?;

    Ok((final_code_svg_content, final_timeline_svg_content))
}

/// Write SVGs to `output_dir` (must end with `/` or use Path — paths are `output_dir/vis_code.svg`).
pub fn render_svg_files(
    input_dir: &str,
    output_dir: &str,
    viz_name: &str,
    visualization_data: &mut VisualizationData,
) -> Result<(), RustvizError> {
    let annotated = utils::read_file_to_string(format!("{}annotated_source.rs", input_dir))
        .map_err(|e| RustvizError::Io(format!("annotated_source.rs: {}", e)))?;
    let source_plain = utils::read_file_to_string(format!("{}source.rs", output_dir))
        .map_err(|e| RustvizError::Io(format!("source.rs: {}", e)))?;

    let (code, tl) = render_svg_strings(&annotated, &source_plain, viz_name, visualization_data)?;

    let code_image_file_path = format!("{}vis_code.svg", output_dir);
    let timeline_image_file_path = format!("{}vis_timeline.svg", output_dir);
    utils::create_and_write_to_file(&code, code_image_file_path);
    utils::create_and_write_to_file(&tl, timeline_image_file_path);
    Ok(())
}

/// Backward-compatible entry: `input_path` is the `input/` directory, `output_path` the example output directory.
pub fn render_svg(
    input_path: &String,
    output_path: &String,
    visualization_data: &mut VisualizationData,
) {
    if let Err(e) = render_svg_files(input_path, output_path, input_path.as_str(), visualization_data) {
        eprintln!("{}", e);
    }
}

fn preprocess_viz_data(visualization_data: &mut VisualizationData) {
    for (_, event_vec) in &mut visualization_data.event_line_map {
        event_vec.sort_by(|a, b| {
            ResourceAccessPoint_extract(a)
                .1
                .as_ref()
                .unwrap()
                .hash()
                .cmp(&ResourceAccessPoint_extract(b).1.as_ref().unwrap().hash())
                .then(
                    ResourceAccessPoint_extract(a)
                        .0
                        .as_ref()
                        .unwrap()
                        .hash()
                        .cmp(&ResourceAccessPoint_extract(b).0.as_ref().unwrap().hash()),
                )
        });
    }

    for (line_number, event) in visualization_data.preprocess_external_events.clone() {
        let mut extra_line: usize = 0;
        for (info_line_number, event_vec) in &visualization_data.event_line_map {
            if info_line_number < &line_number {
                extra_line += event_vec.len() - 1;
            } else {
                break;
            }
        }
        let final_line_num = line_number + extra_line;
        visualization_data.append_processed_external_event(event, final_line_num);
    }

    let mut event_line_map_replace = std::collections::BTreeMap::new();
    let mut extra_line_sum = 0;
    for (line_number, event_vec) in &visualization_data.event_line_map {
        event_line_map_replace.insert(line_number + extra_line_sum, event_vec.clone());
        extra_line_sum += event_vec.len() - 1;
    }
    visualization_data.event_line_map = event_line_map_replace;
}
