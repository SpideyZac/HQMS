use crate::queue::Queue;

pub fn generate_player_leave(queues: &Vec<Queue>, out_file: &str) {
    let mut content = include_str!("templates/PlayerLeave/PlayerLeave.htsl").to_string();

    let mut queueplats_content = String::new();
    let mut playings_content = String::new();
    let mut logics_content = String::new();

    for q in queues.iter() {
        for p in q.platforms.iter() {
            let mut queueplat_template = include_str!("templates/basics/queueplat.htsl")
                .to_string()
                .replace("\\n", "\n");

            queueplat_template = queueplat_template.replace("%queueid%", &q._id.to_string());
            queueplat_template = queueplat_template.replace("%platid%", &p._id.to_string());

            queueplats_content.push_str(&queueplat_template);
        }
    }

    for q in queues.iter() {
        for p in q.platforms.iter() {
            let mut playing_template = include_str!("templates/basics/playing.htsl")
                .to_string()
                .replace("\\n", "\n");

            playing_template = playing_template.replace("%queueid%", &q._id.to_string());
            playing_template = playing_template.replace("%platid%", &p._id.to_string());

            playings_content.push_str(&playing_template);
        }
    }

    let temp_logic = include_str!("templates/PlayerLeave/logic.htsl")
        .to_string()
        .replace("\\n", "\n");
    let conditions_count = temp_logic.matches("if").count();
    let mut current_conditions = 0;
    let mut count = content.matches("if").count();
    for q in queues.iter() {
        for p in q.platforms.iter() {
            if current_conditions + conditions_count > 15 {
                logics_content.push_str(
                    format!(
                        "function \"PlayerLeave_{}\" true\ngoto function \"PlayerLeave_{}\"\n",
                        count, count
                    )
                    .as_str(),
                );
                count += 1;
                current_conditions = 0;
            }
            current_conditions += conditions_count;
            let mut logic_template = include_str!("templates/PlayerLeave/logic.htsl")
                .to_string()
                .replace("\\n", "\n");

            logic_template = logic_template.replace("%queueid%", &q._id.to_string());
            logic_template = logic_template.replace("%platid%", &p._id.to_string());

            logics_content.push_str(&logic_template);
        }
    }

    content = content.replace("%queueplats%", &queueplats_content);
    content = content.replace("%playings%", &playings_content);
    content = content.replace("%logics%", &logics_content);

    std::fs::write(out_file, content).expect("Unable to write file");
}
