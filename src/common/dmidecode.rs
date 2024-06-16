
pub fn get_dmidecode_data(args: &[&str]) {
    let dmidecode_cmd: String = get_dmidecode_program();
    let output = std::process::Command::new(dmidecode_cmd)
        .args(&*args)
        // .current_dir(cwd)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output();
    match output {
        Ok(output) => {
            let stdout = output.stdout.to_vec();
            let stderr = output.stderr.to_vec();
            let exit_code = output.status.code();
        },
        Err(e) => {
            log::error!("Failed to run command: {}", e);
            let stdout: Vec<String> = Vec::new();
            let stderr = format!("{}", e).as_bytes().to_vec();
            let exit_code = Some(2);
        },
    }

    // parse now
}

#[cfg(target_os = "windows")]
fn get_dmidecode_program() -> String {
    return String::from("dmidecode.exe");
}

#[cfg(not(target_os = "windows"))]
fn get_dmidecode_program() -> String {
    return String::from("dmidecode");
}

