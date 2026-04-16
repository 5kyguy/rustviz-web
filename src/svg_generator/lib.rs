pub mod data;
pub mod error;

/// Vertical spacing between logical source lines in SVG output (pixels). Same as [`data::LINE_SPACE`].
#[inline]
pub fn line_step_px() -> i32 {
    data::LINE_SPACE as i32
}
pub mod hover_messages;
pub mod infer;
pub mod parse;
pub mod svg_frontend;

use crate::data::VisualizationData;
use crate::error::RustvizError;
use std::collections::BTreeMap;

/// Full pipeline: parse `main.rs` content and render both SVGs (no disk I/O).
pub fn render_rustviz_from_strings(
    main_rs: &str,
    annotated_source_rs: &str,
    source_rs: &str,
    viz_name: &str,
) -> Result<(String, String), RustvizError> {
    let (rest, line_num, var_map) = parse::parse_vars_to_map_str(main_rs)?;
    let events = parse::extract_events(&rest, line_num)?;
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
        preprocess_external_events: Vec::new(),
        event_line_map: BTreeMap::new(),
    };
    parse::add_events(&mut vd, var_map, events)?;
    svg_frontend::svg_generation::render_svg_strings(annotated_source_rs, source_rs, viz_name, &mut vd)
}

/// Source-only pipeline: infer RustViz definitions/events from plain Rust and render SVGs.
pub fn render_rustviz_from_source(
    source_rs: &str,
    viz_name: &str,
) -> Result<(String, String), RustvizError> {
    let (main_rs, annotated_source_rs, source_plain_rs) = infer::build_inferred_inputs(source_rs);
    render_rustviz_from_strings(&main_rs, &annotated_source_rs, &source_plain_rs, viz_name)
}

/// Same as [`render_rustviz_from_strings`] but returns one combined SVG (code + timeline).
pub fn render_rustviz_from_strings_combined(
    main_rs: &str,
    annotated_source_rs: &str,
    source_rs: &str,
    viz_name: &str,
) -> Result<String, RustvizError> {
    let (rest, line_num, var_map) = parse::parse_vars_to_map_str(main_rs)?;
    let events = parse::extract_events(&rest, line_num)?;
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
        preprocess_external_events: Vec::new(),
        event_line_map: BTreeMap::new(),
    };
    parse::add_events(&mut vd, var_map, events)?;
    svg_frontend::svg_generation::render_combined_svg_string(
        annotated_source_rs,
        source_rs,
        viz_name,
        &mut vd,
    )
}

/// Source-only combined SVG for playground-style UIs.
pub fn render_rustviz_from_source_combined(
    source_rs: &str,
    viz_name: &str,
) -> Result<String, RustvizError> {
    let (main_rs, annotated_source_rs, source_plain_rs) = infer::build_inferred_inputs(source_rs);
    render_rustviz_from_strings_combined(
        &main_rs,
        &annotated_source_rs,
        &source_plain_rs,
        viz_name,
    )
}