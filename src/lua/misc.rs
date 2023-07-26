use mlua::{Lua, MultiValue, Result, Table, ThreadStatus, Value};

use crate::util::pretty_format_multi_value;

pub fn print(_: &Lua, args: MultiValue) -> Result<()> {
    println!(
        "LUA DEBUG: {}",
        args.iter()
            .map(|value| format!("{}", pretty_print_lvalue(value, None)))
            .collect::<Vec<_>>()
            .join(", ")
    );

    Ok(())
}

fn pretty_print_thread_state(state: ThreadStatus) -> String {
    let mut output = String::new();
    match state {
        ThreadStatus::Resumable => output.push_str("Resumable"),
        ThreadStatus::Unresumable => output.push_str("Unresumable"),
        ThreadStatus::Error => output.push_str("Error"),
    }
    output
}

fn pretty_print_table(val: &Table, depth: Option<i32>) -> String {
    let mut output = String::new();
    let d = depth.unwrap_or(0);
    output.push_str(
        format!(
            "{}{}{{\n",
            if d != 0 { "\n" } else { "" },
            "\t".repeat(d as usize)
        )
        .as_str(),
    );
    for pog in val.clone().pairs::<Value, Value>() {
        if let (key, value) = pog.unwrap() {
            let pretty_key = pretty_print_lvalue(&key, Some(d + 1));
            let pretty_value = pretty_print_lvalue(&value, Some(d + 1));
            output.push_str(&format!(
                "{}{} : {},\n",
                "\t".repeat((d + 1) as usize),
                pretty_key,
                pretty_value
            ));
        }
    }
    output.push_str(format!("{}}}", "\t".repeat(d as usize)).as_str());
    output
}

pub fn pretty_print_lvalue(val: &Value, depth: Option<i32>) -> String {
    let mut output = String::new();

    match val {
        Value::Nil => output = "nil".into(),
        Value::Boolean(v) => {
            if v == &true {
                output = "true".into()
            } else {
                output = "false".into()
            }
        }
        Value::LightUserData(_) => todo!(),
        Value::Integer(i) => output = i.to_string(),
        Value::Number(f) => output = f.to_string(),
        Value::String(s) => output = s.to_str().unwrap().into(),
        Value::Table(t) => output = pretty_print_table(t, depth),
        Value::Function(f) => {
            let finfo = f.info();
            output = format!(
                "({what}) fn {name} @ {from}-{to}",
                what =
                    String::from_utf8(finfo.what.unwrap_or(vec!['C' as u8])).unwrap_or("C".into()),
                from = finfo.line_defined,
                to = finfo.line_defined,
                name = String::from_utf8(finfo.name.unwrap_or(vec![
                    'u' as u8, 'n' as u8, 'k' as u8, 'n' as u8, 'o' as u8, 'w' as u8, 'n' as u8,
                ]))
                .unwrap_or("<unknown>".into())
            )
        }
        Value::Thread(t) => output = format!("thread<{}>", pretty_print_thread_state(t.status())),
        Value::UserData(ud) => {
            // println!("{:?}",ud.get_metatable().unwrap().pairs::<Value>().into_iter().collect::<Vec<_>>());
            // ud.
            let meta_name: String = ud
                .get_metatable()
                .unwrap()
                .get("__name")
                .unwrap_or("unknown".to_string());
            output = format!("userdata<{}> : unkown p", meta_name);
        }
        Value::Vector(x, y, z) => output = format!("vector<{}, {}, {}>", x, y, z),
        Value::Error(e) => output = format!("Error<{}>", e),
    }
    output
}
