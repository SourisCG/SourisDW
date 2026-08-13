use souris_dw::deps::DepStatus;

#[test]
fn test_dep_status_serialization() {
    let status = DepStatus {
        name: "test-dep".into(),
        installed: true,
        version: Some("1.2.3".into()),
        path: "/usr/local/bin/test-dep".into(),
        latest: None,
        update_available: false,
    };

    let json = serde_json::to_string(&status).unwrap();
    assert!(json.contains("test-dep"));
    assert!(json.contains("true"));
    assert!(json.contains("1.2.3"));

    let deserialized: DepStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, "test-dep");
    assert!(deserialized.installed);
    assert_eq!(deserialized.version.unwrap(), "1.2.3");
}

#[test]
fn test_dep_status_serialization_not_installed() {
    let status = DepStatus {
        name: "missing".into(),
        installed: false,
        version: None,
        path: "/usr/bin/missing".into(),
        latest: None,
        update_available: false,
    };

    let json = serde_json::to_string(&status).unwrap();
    assert!(json.contains("missing"));

    let deserialized: DepStatus = serde_json::from_str(&json).unwrap();
    assert!(!deserialized.installed);
    assert!(deserialized.version.is_none());
}
