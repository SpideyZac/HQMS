use crate::queue::Queue;

pub fn generate_leave_region(queues: &Vec<Queue>, out_file_base: &str) {
    for q in queues.iter() {
        for p in q.platforms.iter() {
            let mut content = include_str!("templates/LeaveRegion/LeaveRegion.htsl").to_string();

            content = content.replace(
                "%queueplat%",
                format!(
                    "define q{}p{} globalstat \"q{}p{}\"\n",
                    q._id, p._id, q._id, p._id
                )
                .as_str(),
            );
            content = content.replace(
                "%playing%",
                format!(
                    "define playing1q{}p{} stat \"playing1q{}p{}\"\ndefine playing2q{}p{} stat\"playing2q{}p{}\"\n",
                    q._id, p._id, q._id, p._id, q._id, p._id, q._id, p._id
                )
                .as_str(),
            );

            let mut logic_template = include_str!("templates/LeaveRegion/logic.htsl")
                .to_string()
                .replace("\\n", "\n");

            logic_template = logic_template.replace("%queueid%", &q._id.to_string());
            logic_template = logic_template.replace("%platid%", &p._id.to_string());

            content = content.replace("%logic%", &logic_template);

            std::fs::write(
                format!("{}/LeaveRegion_{}_{}.htsl", out_file_base, q._id, p._id),
                content,
            )
            .expect("Unable to write file");
        }
    }
}
