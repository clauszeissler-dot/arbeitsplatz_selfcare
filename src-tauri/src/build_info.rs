//! Build-Marker (A-030): macht sichtbar, ob die App ein Release oder ein lokaler Build ist.

#[derive(Clone, serde::Serialize)]
pub struct BuildInfo {
    pub version: String,
    pub build: String,
}

/// Erlaubt nur die Marker, die unsere Skripte setzen können (`release`, `ci`,
/// `local-<hash>[-dirty]`); alles andere ist ein Entwicklungsbuild.
pub fn build_marker(raw: Option<&str>) -> String {
    match raw {
        Some(value)
            if !value.is_empty()
                && value.len() <= 40
                && value
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_')) =>
        {
            value.to_string()
        }
        _ => "dev".to_string(),
    }
}

pub fn current() -> BuildInfo {
    BuildInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        build: build_marker(option_env!("LOTSE_BUILD")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_markers_pass_through() {
        assert_eq!(build_marker(Some("release")), "release");
        assert_eq!(build_marker(Some("ci")), "ci");
        assert_eq!(build_marker(Some("local-ce807fb")), "local-ce807fb");
        assert_eq!(
            build_marker(Some("local-ce807fb-dirty")),
            "local-ce807fb-dirty"
        );
    }

    #[test]
    fn missing_or_unsafe_markers_become_dev() {
        assert_eq!(build_marker(None), "dev");
        assert_eq!(build_marker(Some("")), "dev");
        assert_eq!(build_marker(Some("local abc")), "dev");
        assert_eq!(build_marker(Some("<script>")), "dev");
        assert_eq!(build_marker(Some(&"x".repeat(41))), "dev");
    }

    #[test]
    fn current_reports_the_crate_version() {
        assert_eq!(current().version, env!("CARGO_PKG_VERSION"));
    }
}
