use crate::queue::Queue;

pub fn generate_player_join(queues: &Vec<Queue>, out_file: &str) {
    let mut content = include_str!("templates/PlayerJoin/PlayerJoin.htsl").to_string();

    let mut inqueues_content = String::new();
    let mut playings_content = String::new();
    let mut logics_content = String::new();

    for q in queues.iter() {
        let mut inqueue_template = include_str!("templates/basics/inqueue.htsl")
            .to_string()
            .replace("\\n", "\n");

        inqueue_template = inqueue_template.replace("%queueid%", &q._id.to_string());

        inqueues_content.push_str(&inqueue_template);
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

    let temp_logic = include_str!("templates/PlayerJoin/logic.htsl")
        .to_string()
        .replace("\\n", "\n");
    let conditions_count = temp_logic.matches("if").count();
    let mut current_conditions = 0;
    let mut count = content.matches("if").count();
    for q in queues.iter() {
        logics_content.push_str(format!("inqueue{} = 0\n", q._id).as_str());
        for p in q.platforms.iter() {
            if current_conditions + conditions_count > 15 {
                logics_content.push_str(
                    format!(
                        "function \"PlayerJoin_{}\" true\ngoto function \"PlayerJoin_{}\"\n",
                        count, count
                    )
                    .as_str(),
                );
                count += 1;
                current_conditions = 0;
            }
            let mut logic_template = include_str!("templates/PlayerJoin/logic.htsl")
                .to_string()
                .replace("\\n", "\n");

            logic_template = logic_template.replace("%queueid%", &q._id.to_string());
            logic_template = logic_template.replace("%platid%", &p._id.to_string());

            logics_content.push_str(&logic_template);

            current_conditions += 1;
            if current_conditions < conditions_count {
                content = content.replace("if", &format!("if{}", current_conditions));
            }
        }
    }

    content = content.replace("%inqueues%", &inqueues_content);
    content = content.replace("%playings%", &playings_content);
    content = content.replace("%logics%", &logics_content);

    std::fs::write(out_file, content).expect("Unable to write file");
}
