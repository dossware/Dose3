pub fn check_and_apply_updates()  {
    #[cfg(target_os = "windows")]
    {
        let cmd = "powershell -Command \"irm https://dose3.dossware.com/install.ps1 | iex\"";

        let status = std::process::Command::new("cmd")
            .args(["/C", cmd])
            .output();

        match status {
            Ok(output) => {
                if output.status.success() {
                    println!("Update applied successfully.");
                } else {
                    eprintln!("Couldn't apply update.");
                }
            }
            Err(e) => eprintln!("❌ Error: {}", e),
        }
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let cmd = "curl -fsSL https://dose3.dossware.com/install.sh | sh";
        
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .status();
        
        match status {
            Ok(out) => {
                if out.success() {
                    println!("Update applied successfully.");
                } else {
                    eprintln!("Couldn't apply update.");
                }
            }
            Err(e) => eprintln!("❌ Error: {}", e),
        }
    }
}