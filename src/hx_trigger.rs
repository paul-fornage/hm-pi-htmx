use serde_json::json;

pub struct HxTrigger {
    pub event: &'static str,
    pub target: &'static str,
}
impl HxTrigger {
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            self.event: { "target": self.target }
        })
    }

    pub fn list_to_json(targets: &[HxTrigger]) -> serde_json::Map<String, serde_json::Value> {
        let mut map = serde_json::Map::with_capacity(targets.len());
        for t in targets {
            map.insert(
                t.event.to_string(), json!({ "target": t.target })
            );
        }
        map
    }
}