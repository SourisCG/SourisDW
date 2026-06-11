use souris_dw::deps::DepStatus;

#[test]
fn test_dep_status_default_fields() {
    let status = DepStatus {
        name: "test".into(),
        installed: true,
        version: Some("1.0".into()),
        path: "/usr/bin/test".into(),
    };
    assert_eq!(status.name, "test");
    assert!(status.installed);
    assert_eq!(status.version.unwrap(), "1.0");
    assert_eq!(status.path, "/usr/bin/test");
}

#[test]
fn test_dep_status_not_installed() {
    let status = DepStatus {
        name: "missing".into(),
        installed: false,
        version: None,
        path: "/usr/bin/missing".into(),
    };
    assert!(!status.installed);
    assert!(status.version.is_none());
}
