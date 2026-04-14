use crate::data::{
    ExternalEvent, Function, MutRef, Owner, ResourceAccessPoint, StaticRef, Struct, VisualizationData,
    Visualizable,
};
use crate::error::RustvizError;
use std::collections::HashMap;

const BEGIN_LINE: &str = "/* --- BEGIN Variable Definitions ---";
const END_LINE: &str = "--- END Variable Definitions --- */";

/// Parse `main.rs` content: variable block + remainder for event extraction.
pub fn parse_vars_to_map_str(
    main_rs: &str,
) -> Result<(Vec<String>, u64, HashMap<String, ResourceAccessPoint>), RustvizError> {
    let lines: Vec<String> = main_rs.lines().map(|s| s.to_string()).collect();
    if lines.is_empty() || lines[0] != BEGIN_LINE {
        return Err(RustvizError::Parse(
            "Uh oh! First line must be `/* --- BEGIN Variable Definitions ---`.".into(),
        ));
    }

    let mut i = 1usize;
    let mut vars_string = String::new();
    let mut num_lines: u64 = 2;

    while i < lines.len() {
        let line = &lines[i];
        if line == END_LINE {
            break;
        }
        num_lines += 1;
        vars_string.push_str(line);
        i += 1;
    }

    if i >= lines.len() || lines[i] != END_LINE {
        return Err(RustvizError::Parse(
            "Something went wrong! Do not remove BEGIN and END statements!".into(),
        ));
    }

    i += 1;
    let rest: Vec<String> = lines[i..].to_vec();

    let vars: Vec<String> = vars_string
        .split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let map = vec_to_map(vars)?;
    Ok((rest, num_lines, map))
}

fn vec_to_map(
    vars_str: Vec<String>,
) -> Result<HashMap<String, ResourceAccessPoint>, RustvizError> {
    let mut vars_map = HashMap::<String, ResourceAccessPoint>::new();
    let mut hash: u64 = 1;

    for v in vars_str.iter() {
        let fields: Vec<&str> = v
            .split(|c| c == ' ' || c == ',' || c == '{' || c == '}')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if fields.is_empty() || fields.len() < 2 {
            print_var_usage_error(&fields)?;
        }

        let name = (if fields.len() > 2 { fields[2] } else { fields[1] }).to_string();

        match (fields[0], fields.len()) {
            ("Owner", 2) | ("Owner", 3) => {
                vars_map.insert(
                    name,
                    ResourceAccessPoint::Owner(Owner {
                        hash,
                        name: get_name_field(&fields),
                        is_mut: get_mut_qualifier(&fields)?,
                    }),
                );
            }
            ("MutRef", 2) | ("MutRef", 3) => {
                vars_map.insert(
                    name,
                    ResourceAccessPoint::MutRef(MutRef {
                        hash,
                        name: get_name_field(&fields),
                        is_mut: get_mut_qualifier(&fields)?,
                    }),
                );
            }
            ("StaticRef", 2) | ("StaticRef", 3) => {
                vars_map.insert(
                    name,
                    ResourceAccessPoint::StaticRef(StaticRef {
                        hash,
                        name: get_name_field(&fields),
                        is_mut: get_mut_qualifier(&fields)?,
                    }),
                );
            }
            ("Function", 2) => {
                vars_map.insert(
                    name,
                    ResourceAccessPoint::Function(Function {
                        hash,
                        name: String::from(fields[1]),
                    }),
                );
            }
            ("Struct", _) => get_structs(&mut hash, &fields, &mut vars_map)?,
            _ => {
                print_var_usage_error(&fields)?;
            }
        }

        hash += 1;
    }

    Ok(vars_map)
}

pub fn extract_events(
    lines: &[String],
    main_line: u64,
) -> Result<Vec<(u64, String)>, RustvizError> {
    let mut events: Vec<(u64, String)> = Vec::new();
    let (mut block_str, mut block) = (String::new(), false);
    let (mut line_begin, mut line_end) = (0, 0);

    for (lnum, line_string) in lines.iter().enumerate() {
        if block {
            if line_string.find("!{").is_some() {
                delimitation_err(line_begin + main_line)?;
            }
            if let Some(j) = line_string.find('}') {
                block_str.push_str(&line_string[..j]);
                for s in block_str.split(',') {
                    events.push((line_begin, s.trim().to_string()));
                }
                block_str.clear();
                block = false;
                line_end = lnum as u64 + 1;
            } else {
                block_str += line_string.trim();
            }
        } else if let Some(i) = line_string.rfind("!{") {
            if let Some(j) = line_string[i..].rfind('}') {
                let evt_str = line_string[i + 2..i + j].trim();
                let diff = line_end - line_begin;
                events.push((lnum as u64 - diff + 1, evt_str.to_string()));
            } else {
                block = true;
                line_begin = lnum as u64 + 1;
                block_str += &line_string[i + 2..];
            }
        }
    }

    if block {
        delimitation_err(line_begin + main_line)?;
    }

    let flat: Vec<(u64, String)> = events
        .iter()
        .flat_map(|(lnum, evts)| {
            evts.split(',')
                .map(|s| (*lnum, s.trim().to_string()))
                .filter(|e| !e.1.is_empty())
                .collect::<Vec<_>>()
        })
        .collect();

    Ok(flat)
}

pub fn add_events(
    vd: &mut VisualizationData,
    vars: HashMap<String, ResourceAccessPoint>,
    events: Vec<(u64, String)>,
) -> Result<(), RustvizError> {
    for event in events {
        let split: Vec<String> = event
            .1
            .split("->")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let mut field: Vec<&str> = Vec::new();
        if split.len() == 1 {
            let idx = split[0]
                .find('(')
                .ok_or_else(|| RustvizError::Parse(event_usage_err()))?;
            field.push(&split[0][..idx]);
            field.push(&split[0][idx + 1..split[0].len() - 1]);
        } else if split.len() == 2 {
            let idx = split[0]
                .find('(')
                .ok_or_else(|| RustvizError::Parse(event_usage_err()))?;
            field.push(&split[0][..idx]);
            field.push(&split[0][idx + 1..]);
            field.push(&split[1][..split[1].len() - 1]);
        } else {
            return Err(RustvizError::Parse(event_usage_err()));
        }

        for f in &field {
            if f.is_empty() {
                return Err(RustvizError::Parse(event_usage_err()));
            }
        }

        match field[0] {
            "Bind" => vd.append_external_event(
                ExternalEvent::Bind {
                    from: get_resource(&vars, "None")?,
                    to: get_resource(&vars, field[1])?,
                },
                &(event.0 as usize),
            ),
            "Copy" => vd.append_external_event(
                ExternalEvent::Copy {
                    from: get_resource(&vars, field[1])?,
                    to: get_resource(&vars, field[2])?,
                },
                &(event.0 as usize),
            ),
            "Move" => vd.append_external_event(
                ExternalEvent::Move {
                    from: get_resource(&vars, field[1])?,
                    to: get_resource(&vars, field[2])?,
                },
                &(event.0 as usize),
            ),
            "StaticBorrow" => vd.append_external_event(
                ExternalEvent::StaticBorrow {
                    from: get_resource(&vars, field[1])?,
                    to: get_resource(&vars, field[2])?,
                },
                &(event.0 as usize),
            ),
            "MutableBorrow" => vd.append_external_event(
                ExternalEvent::MutableBorrow {
                    from: get_resource(&vars, field[1])?,
                    to: get_resource(&vars, field[2])?,
                },
                &(event.0 as usize),
            ),
            "StaticDie" => vd.append_external_event(
                ExternalEvent::StaticDie {
                    from: get_resource(&vars, field[1])?,
                    to: get_resource(&vars, field[2])?,
                },
                &(event.0 as usize),
            ),
            "MutableDie" => vd.append_external_event(
                ExternalEvent::MutableDie {
                    from: get_resource(&vars, field[1])?,
                    to: get_resource(&vars, field[2])?,
                },
                &(event.0 as usize),
            ),
            "PassByStaticReference" => vd.append_external_event(
                ExternalEvent::PassByStaticReference {
                    from: get_resource(&vars, field[1])?,
                    to: get_resource(&vars, field[2])?,
                },
                &(event.0 as usize),
            ),
            "PassByMutableReference" => vd.append_external_event(
                ExternalEvent::PassByMutableReference {
                    from: get_resource(&vars, field[1])?,
                    to: get_resource(&vars, field[2])?,
                },
                &(event.0 as usize),
            ),
            "InitRefParam" => vd.append_external_event(
                ExternalEvent::InitRefParam {
                    param: get_resource(&vars, field[1])?
                        .ok_or_else(|| RustvizError::Parse("Expected Some variable, found None!".into()))?,
                },
                &(event.0 as usize),
            ),
            "InitOwnerParam" => vd.append_external_event(
                ExternalEvent::Move {
                    from: get_resource(&vars, "None")?,
                    to: get_resource(&vars, field[1])?,
                },
                &(event.0 as usize),
            ),
            "GoOutOfScope" => vd.append_external_event(
                ExternalEvent::GoOutOfScope {
                    ro: get_resource(&vars, field[1])?
                        .ok_or_else(|| RustvizError::Parse("Expected Some variable, found None!".into()))?,
                },
                &(event.0 as usize),
            ),
            _ => {
                return Err(RustvizError::Parse(format!(
                    "{} is not a valid event.\n{}",
                    field[0],
                    event_usage_err()
                )));
            }
        }
    }
    Ok(())
}

fn get_resource(
    vars: &HashMap<String, ResourceAccessPoint>,
    name: &str,
) -> Result<Option<ResourceAccessPoint>, RustvizError> {
    if name == "None" {
        Ok(None)
    } else {
        match vars.get(name) {
            Some(res) => Ok(Some(res.clone())),
            None => Err(RustvizError::Parse(format!(
                "Variable '{}' does not exist! Name must match definition.",
                name
            ))),
        }
    }
}

fn get_name_field(fields: &[&str]) -> String {
    (if fields.len() == 2 {
        fields[1]
    } else {
        fields[2]
    })
    .to_string()
}

fn get_mut_qualifier(fields: &[&str]) -> Result<bool, RustvizError> {
    if fields.len() == 2 {
        Ok(false)
    } else if fields[1] == "mut" {
        Ok(true)
    } else {
        Err(RustvizError::Parse(format!(
            "Did not understand qualifier '{}' of variable '{}'! Field must either be empty or 'mut'.",
            fields[1], fields[2]
        )))
    }
}

fn get_structs(
    hash: &mut u64,
    fields: &[&str],
    vars_map: &mut HashMap<String, ResourceAccessPoint>,
) -> Result<(), RustvizError> {
    let b = fields[1] == "mut";
    let parent_name = (if b { fields[2] } else { fields[1] }).to_string();

    vars_map.insert(
        parent_name.clone(),
        ResourceAccessPoint::Struct(Struct {
            owner: *hash,
            hash: *hash,
            name: parent_name.clone(),
            is_mut: b,
            is_member: false,
        }),
    );

    let owner_hash = *hash;
    let mut idx = if b { 3 } else { 2 };
    while idx < fields.len() {
        *hash += 1;
        let cond = fields[idx] == "mut";
        let v_name = parent_name.clone()
            + "."
            + (if cond {
                if idx + 1 >= fields.len() {
                    return Err(RustvizError::Parse(
                        "Expected variable name after 'mut' qualifier, found nothing!".into(),
                    ));
                }
                fields[idx + 1]
            } else {
                fields[idx]
            });

        vars_map.insert(
            v_name.clone(),
            ResourceAccessPoint::Struct(Struct {
                owner: owner_hash,
                hash: *hash,
                name: v_name,
                is_mut: cond,
                is_member: true,
            }),
        );

        idx = if cond { idx + 2 } else { idx + 1 };
    }
    Ok(())
}

fn print_var_usage_error(fields: &[&str]) -> Result<(), RustvizError> {
    Err(RustvizError::Parse(format!(
        "Incorrect variable formatting '{}'!\nUsage (':' denotes optional field):\n\tOwner <:mut> <name>\n\tMutRef <:mut> <name>\n\tStaticRef <:mut> <name>\n\tFunction <name>",
        fields.join(" ")
    )))
}

fn event_usage_err() -> String {
    String::from(
        "ExternalEvents Usage:\n\tFormat: <event_name>(<from> -> <to>)\n\t    e.g.: // !{ PassByMutableReference(a->Some_Function()), ... }\n\tNote: GoOutOfScope and InitRefParam require only the <from> parameter\n\t    e.g.: // !{ GoOutOfScope(x) }",
    )
}

fn delimitation_err(line_num: u64) -> Result<(), RustvizError> {
    Err(RustvizError::Parse(format!(
        "Found unterminated delimitation on line {}! Please close with }}.",
        line_num
    )))
}
