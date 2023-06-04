#[cfg(test)]
mod tests {
    use std::vec;

    use encoder_lib::command::Command::*;

    #[test]
    fn serialize() {
        let commands = vec![Left(10), GripperClose(20)];
        let json = r#"[{"action":"Left","times":10},{"action":"GripperClose","times":20}]"#;

        assert_eq!(json, serde_json::to_string(&commands).unwrap());
    }
}
