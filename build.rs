use std::{env, io::Result, time::SystemTime};
fn main() -> Result<()> {
    println!("generate proto message");
    let mut config = prost_build::Config::new();
    config.default_package_filename("spotware-message");
    //config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config
        .compile_protos(
            &[
                "openapi-proto-messages/OpenApiMessages.proto",
                "openapi-proto-messages/OpenApiCommonMessages.proto",
            ],
            &["openapi-proto-messages/"],
        )
        .unwrap();
    let key = "OUT_DIR";
    println!("generated! outdir = {}\n", std::env::var(key).unwrap());

    // get build time and commit
    let build_time = chrono::DateTime::<chrono::Utc>::from(SystemTime::now())
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let commit = match std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        Ok(output) => String::from_utf8(output.stdout).expect("Failed to get commit"),
        Err(_) => env::var("GIT_COMMIT").unwrap_or("unknown".to_string()),
    };

    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rustc-env=COMMIT={}", commit);
    Ok(())
}
