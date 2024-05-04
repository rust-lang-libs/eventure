use uuid::Uuid;

pub fn generate_id() -> String {
    String::from(&Uuid::new_v4().to_string()[..6])
}
