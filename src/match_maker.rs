use crate::queue::Queue;

pub fn generate_match_maker(queues: &Vec<Queue>, out_file: &str) {
    let mut content = include_str!("templates/MatchMaker/MatchMaker.htsl").to_string();

    let mut queueplats_content = String::new();
    let mut queuetotals_content = String::new();
    let mut inqueues_content = String::new();
    let mut playings_content = String::new();
    let mut starts_content = String::new();
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
        let mut queuetotal_template = include_str!("templates/basics/queuetotal.htsl")
            .to_string()
            .replace("\\n", "\n");

        queuetotal_template = queuetotal_template.replace("%queueid%", &q._id.to_string());

        queuetotals_content.push_str(&queuetotal_template);
    }

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

    for q in queues.iter() {
        for p in q.platforms.iter() {
            let mut start_template = include_str!("templates/basics/start.htsl")
                .to_string()
                .replace("\\n", "\n");

            start_template = start_template.replace("%queueid%", &q._id.to_string());
            start_template = start_template.replace("%platid%", &p._id.to_string());

            starts_content.push_str(&start_template);
        }
    }

    let temp_logic = include_str!("templates/MatchMaker/logic.htsl")
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
                        "function \"MatchMaker_{}\" true\ngoto function \"MatchMaker_{}\"\n",
                        count, count
                    )
                    .as_str(),
                );
                count += 1;
                current_conditions = 0;
            }
            current_conditions += conditions_count;
            let mut logic_template = include_str!("templates/MatchMaker/logic.htsl")
                .to_string()
                .replace("\\n", "\n");

            logic_template = logic_template.replace("%queueid%", &q._id.to_string());
            logic_template = logic_template.replace("%platid%", &p._id.to_string());
            logic_template = logic_template.replace("%spawn1%", &p.spawn1);
            logic_template = logic_template.replace("%spawn2%", &p.spawn2);

            logics_content.push_str(&logic_template);
        }
    }

    content = content.replace("%queueplats%", &queueplats_content);
    content = content.replace("%queuetotals%", &queuetotals_content);
    content = content.replace("%inqueues%", &inqueues_content);
    content = content.replace("%playings%", &playings_content);
    content = content.replace("%starts%", &starts_content);
    content = content.replace("%logics%", &logics_content);

    std::fs::write(out_file, content).expect("Unable to write file");
}
