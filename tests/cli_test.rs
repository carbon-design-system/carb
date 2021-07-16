use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn init_command() {
    let cwd = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("carb").unwrap();

    cmd.current_dir(cwd.path()).arg("init").assert().success();

    let config_dir = cwd.path().join(".carb");
    let config_file = config_dir.join("config.yml");

    assert!(config_dir.exists());
    assert!(config_file.exists());
}
