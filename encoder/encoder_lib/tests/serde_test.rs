#[cfg(test)]
mod tests {
    use std::vec;

    use encoder_lib::{command::Command::*, encoder::Step};

    #[test]
    fn serialize() {
        let steps = vec![
            Step {
                command: Left,
                times: 10,
            },
            Step {
                command: GripperClose,
                times: 20,
            },
        ];
        let json = r#"[{"command":"Left","times":10},{"command":"GripperClose","times":20}]"#;

        assert_eq!(json, serde_json::to_string(&steps).unwrap());
    }
}
