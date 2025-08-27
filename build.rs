use std::process::Command;

fn main() {
    // Tell Cargo to rerun this build script if anything in src/views/ changes.
    // This ensures that when HTML templates change, the Tailwind CSS build reruns.
    println!("cargo:rerun-if-changed=src/views/");

    // Tailwind build command (independent of platform).
    // Equivalent to: `npx tailwindcss -i ./tailwind.css -o ./static/styles.css`
    let tailwind_command = "npx";
    let tailwind_args = [
        "tailwindcss",
        "-i",
        "./tailwind.css",
        "-o",
        "./static/styles.css",
    ];

    // Choose the correct shell depending on the OS:
    // - Windows: use `cmd /C`
    // - macOS/Linux: use `sh -c`
    #[cfg(target_os = "windows")]
    let mut cmd = {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(tailwind_command).args(&tailwind_args);
        c
    };

    #[cfg(not(target_os = "windows"))]
    let mut cmd = {
        let mut c = Command::new("sh");
        c.arg("-c").arg(format!(
            "{} {}",
            tailwind_command,
            tailwind_args.join(" ")
        ));
        c
    };

    // Run the Tailwind build process
    let result = cmd.output().expect("Unable to run Tailwind build process");

    // If the command failed, surface the error as a Cargo build warning.
    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        println!("cargo:warning=Unable to build CSS!");
        println!("cargo:warning=Output: {stderr}");
    }
}