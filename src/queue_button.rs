use crate::queue::Queue;

pub fn generate_queue_button(queues: &Vec<Queue>, out_file_base: &str) {
    for q in queues.iter() {
        let mut content = include_str!("templates/QueueButton/QueueButton.htsl").to_string();

        content = content.replace(
            "%queuetotal%",
            format!(
                "define queuetotal{} globalstat \"queuetotal{}\"\n",
                q._id, q._id
            )
            .as_str(),
        );
        content = content.replace(
            "%inqueue%",
            format!("define inqueue{} stat \"inqueue{}\"\n", q._id, q._id).as_str(),
        );

        let mut logic_template = include_str!("templates/QueueButton/logic.htsl")
            .to_string()
            .replace("\\n", "\n");
        logic_template = logic_template.replace("%queueid%", &q._id.to_string());
        content = content.replace("%logics%", &logic_template);

        std::fs::write(
            format!("{}/QueueButton_{}.htsl", out_file_base, q._id),
            content,
        )
        .expect("Unable to write file");
    }
}
