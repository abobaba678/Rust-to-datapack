use std::fs;         
use std::path::Path; 
use std::io::Write;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json;

pub fn restart_files() {
    let load = vec!["{","  \"values\": [", "    \"main:init\"", "  ]", "}"];
    let tick = vec!["{","  \"values\": [", "    \"main:tick\"", "  ]", "}"];

    if !Path::new("data").exists() {
        fs::create_dir("data").unwrap();
    }
    fs::create_dir_all("data/main").unwrap();
    fs::create_dir_all("data/main/functions").unwrap();
    fs::create_dir_all("data/main/advancements").unwrap();
    fs::create_dir_all("data/main/recipes").unwrap();
    fs::create_dir_all("data/minecraft/tags/functions").unwrap();
    fs::write("data/minecraft/tags/functions/load.json", load.join("\n")).unwrap();
    fs::write("data/minecraft/tags/functions/tick.json", tick.join("\n")).unwrap();
    fs::write("data/main/functions/init.mcfunction", "").unwrap();
    fs::write("data/main/functions/tick.mcfunction", "").unwrap();
}

pub fn load(content: &str) {
    use std::fs::OpenOptions;

    let mut file = OpenOptions::new()
        .append(true)
        .open("data/main/functions/init.mcfunction")
        .unwrap();

    writeln!(file, "{}", content).unwrap();
}

pub fn tick(content: &str) {
    use std::fs::OpenOptions;

    let mut file = OpenOptions::new()
        .append(true)
        .open("data/main/functions/tick.mcfunction")
        .unwrap();

    writeln!(file, "{}", content).unwrap();
}

pub fn create_fn(name: &str) {
    let path = vec!["data/main/functions/", name, ".mcfunction"];
    fs::write(path.join(""), "").unwrap();
}

pub fn wfn(name: &str, content: &str) {
    use std::fs::OpenOptions;

    let path = vec!["data/main/functions/", name, ".mcfunction"];

    let mut file = OpenOptions::new()
        .append(true)
        .open(path.join(""))
        .unwrap();

    writeln!(file, "{}", content).unwrap();
}

#[derive(Serialize, Deserialize)]
struct KeyItem {
    item: String,
}

#[derive(Serialize, Deserialize)]
struct ResultItem {
    item: String,
}

#[derive(Serialize, Deserialize)]
struct Recipe {
    #[serde(rename = "type")]
    recipe_type: String,
    pattern: Vec<String>,
    key: HashMap<String, KeyItem>,
    result: ResultItem,
}

pub fn recipe_template(
    name: &str,
    pattern: Vec<&str>,
    keys: Vec<(&str, &str)>,
    result: &str,
) {
    let _row_len = pattern[0].len();

    let mut key_map = HashMap::new();
    for (key_char, item) in keys {
        key_map.insert(
            key_char.to_string(),
            KeyItem {
                item: item.to_string(),
            },
        );
    }

    let recipe = Recipe {
        recipe_type: "minecraft:crafting_shaped".to_string(),
        pattern: pattern.iter().map(|s| s.to_string()).collect(),
        key: key_map,
        result: ResultItem {
            item: result.to_string(),
        },
    };

    let json_text = serde_json::to_string_pretty(&recipe).unwrap();

    let path = format!("data/main/recipes/{}.json", name);

    if let Some(parent) = std::path::Path::new(&path).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    
    fs::write(&path, json_text).unwrap();
    println!("Recipe saved to: {}", path);
}

pub fn connect_advancement(
    name: &str,
    name_craft: &str,
    name_fn: &str,
    result: &str,
) {
    let template = format!(
        r#"{{
    "criteria": {{
        "extrarecipes": {{
            "trigger": "minecraft:recipe_unlocked",
            "conditions": {{
                "recipe": "main:{}"
            }}
        }}
    }},
    "rewards": {{
        "function": "main:{}"
    }}
}}"#,
        name_craft, name_fn
    );

    let template2 = format!(
        r#"advancement revoke @s only main:{}
recipe take @s main:{}
clear @s minecraft:knowledge_book
give @p {}"#,
        name, name_craft, result
    );

    let advancement_path = vec!["data/main/advancements/", name, ".json"];
    let function_path = vec!["data/main/functions/", name_fn, ".mcfunction"];

    if let Some(parent) = std::path::Path::new(&advancement_path.join("")).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    if let Some(parent) = std::path::Path::new(&function_path.join("")).parent() {
        fs::create_dir_all(parent).unwrap();
    }

    fs::write(advancement_path.join(""), template).unwrap();
    fs::write(function_path.join(""), template2).unwrap();
}