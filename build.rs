use std::{env, fs, process::Command};

// NOTE: need 'xdotool' and 'libxdo-dev'
// TODO: DRY
// TODO: print make errors for -vv build
// TODO: delete original file after copy
#[cfg(target_os = "windows")]
fn main() {
    let target_dir = env::current_dir().expect("Unable to determine target directory.");
    match env::var("PROFILE") {
        // For some reason destructuring like: Ok("profile") does not work
        Ok(str) if str == "release" => {
            Command::new("MSBuild")
                .arg("./Telekinetic.sln")
                .arg("/property:Configuration=Release")
                .output()
                .expect("Handtracker compilation failed");

            let target_file = target_dir.join("target/release/hand-tracker.exe");
            // TODO: log error instead of panicking here
            fs::copy("hand_tracking/Release/hand-tracker.exe", target_file)
                .expect("Failed to copy executable");
        }
        Ok(str) if str == "debug" => {
            Command::new("MSBuild")
                .arg("./Telekinetic.sln")
                .arg("/property:Configuration=Debug")
                .output()
                .expect("Handtracker compilation failed");

            let target_file = target_dir.join("target/debug/hand-tracker.exe");
            // panic!("Target File: {:?}", target_file.clone()); //debug file location
            fs::copy("hand_tracking/Debug/hand-tracker.exe", target_file)
                .expect("Failed to copy executable");
        }
        _ => panic!("Unable to determine build profile."),
    }
}
#[cfg(not(target_os = "windows"))]
fn main() {
    let target_dir = env::current_dir().expect("Unable to determine target directory.");
    match env::var("PROFILE") {
        // For some reason destructuring like: Ok("profile") does not work
        Ok(str) if str == "release" => {
            Command::new("make")
                .current_dir("hand_tracking/build")
                .output()
                .expect("Handtracker compilation failed");

            let target_file = target_dir.join("target/release/hand-tracker");
            // TODO: log error instead of panicking here
            fs::copy("hand_tracking/hand-tracker", target_file).expect("Failed to copy executable");
        }
        Ok(str) if str == "debug" => {
            Command::new("make")
                .current_dir("hand_tracking/build")
                .output()
                .expect("Handtracker compilation failed");

            let target_file = target_dir.join("target/debug/hand-tracker");
            //panic!("Target File: {:?}", target_file.clone()); //debug file location
            fs::copy("hand_tracking/hand-tracker", target_file).expect("Failed to copy executable");
        }
        _ => panic!("Unable to determine build profile."),
    }
}
