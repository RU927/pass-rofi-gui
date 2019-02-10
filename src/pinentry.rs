use std::process;

pub fn rofi_get_passphrase() -> Option<String> {
    match String::from_utf8(
        process::Command::new("rofi")
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .args(&["-dmenu"])
            .args(&["-input", "/dev/null"])
            .args(&["-lines", "0"])
            .arg("-i") // case-insensitive
            .args(&["-width", "20"])
            .arg("-disable-history")
            .arg("-password")
            .args(&["-p", "passphrase"]) // prompt
            .args(&[
                "-mesg",
                "Please enter the passphrase to unlock the OpenPGP secret key",
            ])
            .spawn()
            .expect("failed to spawn rofi")
            .wait_with_output()
            .expect("failed to read rofi stdout")
            .stdout,
    )
    .expect("failed to read passphrase as utf8")
    .trim()
    {
        "" => None,
        val => Some(val.to_owned()),
    }
}