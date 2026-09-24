use super::{encode_pkg_name_path, normalize_registry_url, to_registry_url};

#[test]
fn unscoped_name_passes_through() {
    assert_eq!(encode_pkg_name_path("lodash"), "lodash");
    assert_eq!(encode_pkg_name_path("acme-helper"), "acme-helper");
    assert_eq!(encode_pkg_name_path("acme_helper"), "acme_helper");
    assert_eq!(encode_pkg_name_path("acme.helper"), "acme.helper");
    assert_eq!(encode_pkg_name_path("acme~legacy"), "acme~legacy");
}

#[test]
fn scoped_name_encodes_slash() {
    assert_eq!(encode_pkg_name_path("@scope/pkg"), "@scope%2Fpkg");
    assert_eq!(encode_pkg_name_path("@pnpm.e2e/hello-world"), "@pnpm.e2e%2Fhello-world");
}

#[test]
fn url_join_normalizes_trailing_slash() {
    assert_eq!(
        to_registry_url("https://registry.npmjs.org/", "@scope/pkg"),
        "https://registry.npmjs.org/@scope%2Fpkg",
    );
    assert_eq!(
        to_registry_url("https://registry.npmjs.org", "@scope/pkg"),
        "https://registry.npmjs.org/@scope%2Fpkg",
    );
}

#[test]
fn normalize_registry_url_drops_only_default_ports() {
    for (input, expected) in [
        (
            "https://registry.example.com:443/package.tgz",
            "https://registry.example.com/package.tgz",
        ),
        ("http://registry.example.com:80/package.tgz", "http://registry.example.com/package.tgz"),
        (
            "https://registry.example.com:8443/package.tgz",
            "https://registry.example.com:8443/package.tgz",
        ),
        (
            "http://registry.example.com:8080/package.tgz",
            "http://registry.example.com:8080/package.tgz",
        ),
        ("https://registry.example.com/package.tgz", "https://registry.example.com/package.tgz"),
        ("http://registry.example.com/package.tgz", "http://registry.example.com/package.tgz"),
        (
            "https://artifactory:443/api/npm/npm-virtual/uuid/-/uuid-9.0.1.tgz",
            "https://artifactory/api/npm/npm-virtual/uuid/-/uuid-9.0.1.tgz",
        ),
        ("invalid-url", "invalid-url"),
    ] {
        assert_eq!(normalize_registry_url(input), expected, "input: {input}");
    }
}
