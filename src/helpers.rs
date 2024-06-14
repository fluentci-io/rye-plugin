use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_rye(version: String) -> Result<String, Error> {
    if !version.is_empty() {
        dag().set_envs(vec![("RYE_VERSION".into(), version)])?;
    }

    let home = dag().get_env("HOME")?;
    let path = dag().get_env("PATH")?;

    dag().set_envs(vec![(
        "PATH".into(),
        format!("{}/.rye/shims:{}", home, path),
    )])?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            &format!("type rye > /dev/null || pkgx curl -sSf https://rye.astral.sh/get | RYE_INSTALL_OPTION=\"--yes\" bash"),
        ])?
        .stdout()?;

    Ok(stdout)
}
