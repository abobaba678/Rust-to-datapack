mod main_commands;

use main_commands::*;

fn radius(radius: i32) {
    for x in -radius..=radius {
        for y in -radius..=radius {
            for z in -radius..=radius {
                let command2 = format!(
                    "execute at @a if block ~{} ~{} ~{} minecraft:carved_pumpkin if block ~{} ~{} ~{} minecraft:soul_sand run fill ~{} ~{} ~{} ~{} ~{} ~{} minecraft:air replace",
                    x, y, z, x, y-1, z, x, y-1, z, x, y, z
                );
                let command = format!(
                    "execute at @a if block ~{} ~{} ~{} minecraft:carved_pumpkin if block ~{} ~{} ~{} minecraft:soul_sand run summon minecraft:wither_skeleton ~{} ~{} ~{}",
                    x, y, z, x, y-1, z, x, y, z
                );
                tick(&command);
                tick(&command2); //popa
            }
        }
    }
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    restart_files();

    create_fn("welcome");
    wfn("welcome", "title @a subtitle {\"text\":\"made by abobaba678 :)\",\"color\":\"red\"}");

    load("title @a title {\"text\":\"RUST CORE USED!\",\"bold\":true,\"color\":\"dark_red\"}");
    load("function main:welcome");
    radius(4);

    recipe_template("biblia", vec!["S"],
        vec![
            ("S", "minecraft:book"),
        ],
        "minecraft:knowledge_book",
    );

    connect_advancement("biblia_advancement", "biblia", "give_biblia_func", "minecraft:player_head{display:{Name:\'{\"text\":\"Библия\",\"color\":\"gold\",\"underlined\":true,\"bold\":true,\"italic\":false}\',Lore:[\'{\"text\":\"ID головы: 1426\",\"color\":\"gray\",\"italic\":false}\','{\"text\":\"\",\"color\":\"blue\",\"italic\":false}\']},SkullOwner:{Id:[I;2080793942,-524468218,-1541115779,1949756395],Properties:{textures:[{Value:\"eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvNmRlNGFiMTI5ZTEzN2Y5ZjRjYmY3MDYwMzE4ZWUxNzQ4ZGMzOWRhOWI1ZDEyOWE4ZGEwZTYxNGUyMzM3NjkzIn19fQ==\"}]}}} 1");

    Ok(())
}

