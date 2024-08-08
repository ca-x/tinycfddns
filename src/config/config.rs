pub struct Auth {
    user: String,
    api_key: String,
}
pub struct DDnsTask {
    zone_name: String,
    dns_record_names: Vec<String>,
}