use extism_pdk::*;
use fluentci_pdk::dag;

use crate::helpers::setup_rye;

pub mod helpers;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let stdout = setup_rye(version)?;
    Ok(stdout)
}

#[plugin_fn]
pub fn fmt(args: String) -> FnResult<String> {
    let version = dag().get_env("RYE_VERSION").unwrap_or_default();
    setup_rye(version)?;

    let stdout = dag()
        .pipeline("fmt")?
        .with_exec(vec!["rye", "fmt", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn build(args: String) -> FnResult<String> {
    let version = dag().get_env("RYE_VERSION").unwrap_or_default();
    setup_rye(version)?;

    let stdout = dag()
        .pipeline("build")?
        .with_exec(vec!["rye", "build", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn fetch(args: String) -> FnResult<String> {
    let version = dag().get_env("RYE_VERSION").unwrap_or_default();
    setup_rye(version)?;

    let stdout = dag()
        .pipeline("fetch")?
        .with_exec(vec!["rye", "fetch", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn init(args: String) -> FnResult<String> {
    let version = dag().get_env("RYE_VERSION").unwrap_or_default();
    setup_rye(version)?;

    let stdout = dag()
        .pipeline("init")?
        .with_exec(vec!["rye", "init", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn install(args: String) -> FnResult<String> {
    let version = dag().get_env("RYE_VERSION").unwrap_or_default();
    setup_rye(version)?;

    let stdout = dag()
        .pipeline("install")?
        .with_exec(vec!["rye", "install", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn publish(args: String) -> FnResult<String> {
    let version = dag().get_env("RYE_VERSION").unwrap_or_default();
    setup_rye(version)?;

    let stdout = dag()
        .pipeline("publish")?
        .with_exec(vec!["rye", "publish", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn run(args: String) -> FnResult<String> {
    let version = dag().get_env("RYE_VERSION").unwrap_or_default();
    setup_rye(version)?;

    let stdout = dag()
        .pipeline("run")?
        .with_exec(vec!["rye", "run", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn toolchain(args: String) -> FnResult<String> {
    let version = dag().get_env("RYE_VERSION").unwrap_or_default();
    setup_rye(version)?;

    let stdout = dag()
        .pipeline("toolchain")?
        .with_exec(vec!["rye", "toolchain", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn tools(args: String) -> FnResult<String> {
    let version = dag().get_env("RYE_VERSION").unwrap_or_default();
    setup_rye(version)?;

    let stdout = dag()
        .pipeline("tools")?
        .with_exec(vec!["rye", "tools", &args])?
        .stdout()?;

    Ok(stdout)
}
