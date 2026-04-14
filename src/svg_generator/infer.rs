use std::collections::{BTreeMap, BTreeSet};

const BEGIN_LINE: &str = "/* --- BEGIN Variable Definitions ---";
const END_LINE: &str = "--- END Variable Definitions --- */";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VarKind {
    Owner { is_mut: bool },
    StaticRef,
    MutRef,
}

/// Build best-effort RustViz inputs from a plain Rust source file.
///
/// The existing RustViz renderer still expects:
/// - `main.rs` style variable definitions
/// - event annotations in `!{ ... }` comments
/// - a separate annotated source for code-panel text
///
/// This helper synthesizes those pieces directly from one source string.
pub fn build_inferred_inputs(source_rs: &str) -> (String, String, String) {
    let mut seen_order: Vec<String> = Vec::new();
    let mut seen_set: BTreeSet<String> = BTreeSet::new();
    let mut kinds: BTreeMap<String, VarKind> = BTreeMap::new();
    let mut events_by_line: BTreeMap<usize, Vec<String>> = BTreeMap::new();

    let mut lines: Vec<String> = source_rs.lines().map(|line| line.to_string()).collect();
    if lines.is_empty() {
        lines.push(String::new());
    }

    for (idx, line) in lines.iter().enumerate() {
        let line_num = idx + 1;
        let trimmed = strip_line_comment(line).trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some((lhs, rhs)) = parse_let_statement(trimmed) {
            let lhs_name = lhs.name;
            let lhs_mut = lhs.is_mut;

            if !seen_set.insert(lhs_name.clone()) {
                push_event(&mut events_by_line, line_num, format!("GoOutOfScope({})", lhs_name));
            } else {
                seen_order.push(lhs_name.clone());
            }

            if let Some(ref_target) = rhs.strip_prefix("&mut ").and_then(first_ident) {
                ensure_seen_owner(
                    ref_target,
                    &mut seen_order,
                    &mut seen_set,
                    &mut kinds,
                );
                kinds.insert(lhs_name.clone(), VarKind::MutRef);
                push_event(
                    &mut events_by_line,
                    line_num,
                    format!("MutableBorrow({}->{})", ref_target, lhs_name),
                );
                continue;
            }

            if let Some(ref_target) = rhs.strip_prefix('&').and_then(first_ident) {
                ensure_seen_owner(
                    ref_target,
                    &mut seen_order,
                    &mut seen_set,
                    &mut kinds,
                );
                kinds.insert(lhs_name.clone(), VarKind::StaticRef);
                push_event(
                    &mut events_by_line,
                    line_num,
                    format!("StaticBorrow({}->{})", ref_target, lhs_name),
                );
                continue;
            }

            if let Some(from_name) = only_ident(rhs) {
                ensure_seen_owner(
                    from_name,
                    &mut seen_order,
                    &mut seen_set,
                    &mut kinds,
                );
                kinds.insert(lhs_name.clone(), VarKind::Owner { is_mut: lhs_mut });
                push_event(
                    &mut events_by_line,
                    line_num,
                    format!("Move({}->{})", from_name, lhs_name),
                );
            } else {
                kinds.insert(lhs_name.clone(), VarKind::Owner { is_mut: lhs_mut });
                push_event(
                    &mut events_by_line,
                    line_num,
                    format!("Bind(None->{})", lhs_name),
                );
            }

            continue;
        }

        if let Some((lhs, rhs)) = parse_simple_assignment(trimmed) {
            if let Some(from_name) = only_ident(rhs) {
                ensure_seen_owner(
                    lhs,
                    &mut seen_order,
                    &mut seen_set,
                    &mut kinds,
                );
                ensure_seen_owner(
                    from_name,
                    &mut seen_order,
                    &mut seen_set,
                    &mut kinds,
                );
                push_event(
                    &mut events_by_line,
                    line_num,
                    format!("Move({}->{})", from_name, lhs),
                );
            }
        }
    }

    // Keep timeline finite: everything eventually leaves scope.
    let final_line = lines.len();
    for name in &seen_order {
        push_event(
            &mut events_by_line,
            final_line,
            format!("GoOutOfScope({})", name),
        );
    }

    let main_rs = build_main_rs(&lines, &seen_order, &kinds, &events_by_line);
    let annotated_source_rs = escape_xml(source_rs);
    (main_rs, annotated_source_rs, source_rs.to_string())
}

fn build_main_rs(
    source_lines: &[String],
    seen_order: &[String],
    kinds: &BTreeMap<String, VarKind>,
    events_by_line: &BTreeMap<usize, Vec<String>>,
) -> String {
    let mut out = String::new();
    out.push_str(BEGIN_LINE);
    out.push('\n');

    for name in seen_order {
        match kinds.get(name).copied().unwrap_or(VarKind::Owner { is_mut: false }) {
            VarKind::Owner { is_mut } => {
                if is_mut {
                    out.push_str(&format!("Owner mut {};\n", name));
                } else {
                    out.push_str(&format!("Owner {};\n", name));
                }
            }
            VarKind::StaticRef => {
                out.push_str(&format!("StaticRef {};\n", name));
            }
            VarKind::MutRef => {
                out.push_str(&format!("MutRef {};\n", name));
            }
        }
    }

    out.push_str(END_LINE);
    out.push('\n');

    for (idx, line) in source_lines.iter().enumerate() {
        let line_num = idx + 1;
        let mut rendered = line.clone();
        if let Some(events) = events_by_line.get(&line_num) {
            if !events.is_empty() {
                rendered.push_str(" // !{ ");
                rendered.push_str(&events.join(", "));
                rendered.push_str(" }");
            }
        }
        out.push_str(&rendered);
        out.push('\n');
    }

    out
}

fn ensure_seen_owner(
    name: &str,
    seen_order: &mut Vec<String>,
    seen_set: &mut BTreeSet<String>,
    kinds: &mut BTreeMap<String, VarKind>,
) {
    if !is_identifier(name) {
        return;
    }
    if seen_set.insert(name.to_string()) {
        seen_order.push(name.to_string());
    }
    kinds
        .entry(name.to_string())
        .or_insert(VarKind::Owner { is_mut: false });
}

fn push_event(events_by_line: &mut BTreeMap<usize, Vec<String>>, line: usize, event: String) {
    events_by_line.entry(line).or_default().push(event);
}

fn strip_line_comment(line: &str) -> &str {
    match line.find("//") {
        Some(i) => &line[..i],
        None => line,
    }
}

struct LetInfo {
    name: String,
    is_mut: bool,
}

fn parse_let_statement(line: &str) -> Option<(LetInfo, &str)> {
    let s = line.trim();
    if !s.starts_with("let ") {
        return None;
    }

    let without_let = s.strip_prefix("let ")?.trim_start();
    let eq_index = without_let.find('=')?;
    let lhs = without_let[..eq_index].trim();
    let rhs = without_let[eq_index + 1..].trim().trim_end_matches(';').trim();
    if rhs.is_empty() {
        return None;
    }

    let lhs_core = lhs.split(':').next()?.trim();
    let (is_mut, name_part) = if let Some(rest) = lhs_core.strip_prefix("mut ") {
        (true, rest.trim())
    } else {
        (false, lhs_core)
    };
    let name = first_ident(name_part)?;
    Some((
        LetInfo {
            name: name.to_string(),
            is_mut,
        },
        rhs,
    ))
}

fn parse_simple_assignment(line: &str) -> Option<(&str, &str)> {
    let s = line.trim();
    if s.starts_with("let ") || s.contains("==") {
        return None;
    }
    let eq_index = s.find('=')?;
    let lhs = s[..eq_index].trim();
    let rhs = s[eq_index + 1..].trim().trim_end_matches(';').trim();
    if rhs.is_empty() || !is_identifier(lhs) {
        return None;
    }
    Some((lhs, rhs))
}

fn first_ident(input: &str) -> Option<&str> {
    let mut end = 0usize;
    for (idx, ch) in input.char_indices() {
        let valid = if idx == 0 {
            ch == '_' || ch.is_ascii_alphabetic()
        } else {
            ch == '_' || ch.is_ascii_alphanumeric()
        };
        if !valid {
            break;
        }
        end = idx + ch.len_utf8();
    }
    if end == 0 {
        None
    } else {
        Some(&input[..end])
    }
}

fn only_ident(input: &str) -> Option<&str> {
    let ident = first_ident(input)?;
    if ident.len() == input.trim().len() && is_identifier(ident) {
        Some(ident)
    } else {
        None
    }
}

fn is_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c == '_' || c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
}

fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}
