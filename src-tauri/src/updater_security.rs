use serde_json::{json, Value};
use tauri::test::{mock_builder, mock_context, noop_assets};
use tauri_plugin_updater::{Error, UpdaterExt};

const CONTROL_PACKAGE: &[u8] = include_bytes!("../../tests/fixtures/updater-control.txt");
const CONTROL_SIGNATURE: &str = include_str!("../../tests/fixtures/updater-control.txt.sig");

fn manifest(version: &str) -> Value {
    json!({
        "version": version,
        "platforms": {
            "macos-universal": {
                "url": "https://example.invalid/Lotse.app.tar.gz",
                "signature": CONTROL_SIGNATURE.trim(),
            }
        }
    })
}

#[test]
fn signed_update_control_passes_while_tampering_and_version_rebinding_fail() {
    let actual_config: Value = serde_json::from_str(include_str!("../tauri.conf.json")).unwrap();
    let mut context = mock_context(noop_assets());
    context.config_mut().plugins.0.insert(
        "updater".into(),
        actual_config["plugins"]["updater"].clone(),
    );
    let app = mock_builder()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .build(context)
        .unwrap();
    let updater = app
        .updater_builder()
        .target("macos-universal")
        .executable_path("/private/tmp/Lotse.app/Contents/MacOS/lotse")
        .build()
        .unwrap();

    let update = updater.restore_update(manifest("0.2.0")).unwrap().unwrap();
    update.verify(CONTROL_PACKAGE).unwrap();

    let protected_state = CONTROL_PACKAGE.to_vec();
    assert!(matches!(
        update.verify(b"manipulated updater package"),
        Err(Error::Minisign(_))
    ));
    assert_eq!(CONTROL_PACKAGE, protected_state);

    let rebound = updater.restore_update(manifest("0.3.0")).unwrap().unwrap();
    assert!(matches!(
        rebound.verify(CONTROL_PACKAGE),
        Err(Error::SignedVersionMismatch { .. })
    ));
    assert_eq!(CONTROL_PACKAGE, protected_state);
}

#[test]
#[ignore = "requires a freshly built and signed release archive"]
fn built_update_archive_passes_the_runtime_verifier_and_tampering_fails() {
    let archive_path = std::env::var("LOTSE_UPDATE_ARCHIVE").unwrap();
    let manifest_path = std::env::var("LOTSE_UPDATE_MANIFEST").unwrap();
    let manifest: Value = serde_json::from_slice(&std::fs::read(manifest_path).unwrap()).unwrap();
    let actual_config: Value = serde_json::from_str(include_str!("../tauri.conf.json")).unwrap();
    let mut context = mock_context(noop_assets());
    context.config_mut().plugins.0.insert(
        "updater".into(),
        actual_config["plugins"]["updater"].clone(),
    );
    let app = mock_builder()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .build(context)
        .unwrap();
    let updater = app
        .updater_builder()
        .target("macos-universal")
        .executable_path("/private/tmp/Lotse.app/Contents/MacOS/lotse")
        .build()
        .unwrap();
    let update = updater.restore_update(manifest).unwrap().unwrap();
    let archive = std::fs::read(archive_path).unwrap();

    update.verify(&archive).unwrap();
    let mut manipulated = archive.clone();
    manipulated[0] ^= 0x01;
    assert!(matches!(
        update.verify(&manipulated),
        Err(Error::Minisign(_))
    ));
    assert_ne!(manipulated, archive);
}
