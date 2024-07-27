use crate::queue::Queue;

pub fn generate_join_queue(queues: &Vec<Queue>, out_file: &str) {
    let mut content = include_str!("templates/JoinQueue/JoinQueue.htsl").to_string();

    let mut queuetotals_content = String::new();
    let mut inqueues_content = String::new();
    let mut logics_content = String::new();

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

    let temp_logic = include_str!("templates/JoinQueue/logic.htsl")
        .to_string()
        .replace("\\n", "\n");
    let conditions_count = temp_logic.matches("if").count();
    let mut current_conditions = 0;
    let mut count = content.matches("if").count();
    for q in queues.iter() {
        if current_conditions + conditions_count > 15 {
            logics_content.push_str(
                format!(
                    "function \"JoinQueue_{}\" true\ngoto function \"JoinQueue_{}\"\n",
                    count, count
                )
                .as_str(),
            );
            count += 1;
            current_conditions = 0;
        }
        current_conditions += conditions_count;
        let mut logic_template = include_str!("templates/JoinQueue/logic.htsl")
            .to_string()
            .replace("\\n", "\n");

        logic_template = logic_template.replace("%queueid%", &q._id.to_string());

        logics_content.push_str(&logic_template);
    }

    content = content.replace("%queuetotals%", &queuetotals_content);
    content = content.replace("%inqueues%", &inqueues_content);
    content = content.replace("%logics%", &logics_content);

    std::fs::write(out_file, content).expect("Unable to write file");
}
