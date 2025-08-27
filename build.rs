use std::process::Command;

fn main() {
    // Re-run build.rs if the input file or tailwind config changes
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=src/input.css");

    // Run Tailwind CLI
    let status = Command::new("npx")
        .args([
            "tailwindcss",
            "-i",
            "./tailwind.css", // input file
            "-o",
            "./static/styles.css", // output file
            "--minify",
        ])
        .status()
        .expect("failed to run tailwindcss build");

    if !status.success() {
        panic!("tailwindcss build failed");
    }
}
