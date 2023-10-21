//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// fn main() {
//     // In tests7, we should set up an environment variable
//     // called `TEST_FOO`. Print in the standard output to let
//     // Cargo do it.
//     let timestamp = std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
//         .as_secs(); // What's the use of this timestamp here?
//     let your_command = format!(
//         "Your command here with {}, please checkout exercises/tests/build.rs",
//         timestamp
//     );
//     println!("cargo:{}", your_command);

//     // In tests8, we should enable "pass" feature to make the
//     // testcase return early. Fill in the command to tell
//     // Cargo about that.
//     let your_command = "Your command here, please checkout exercises/tests/build.rs";
//     println!("cargo:{}", your_command);
// }


#[warn(unused_imports)]
fn main() {
    // In tests7, set up an environment variable called `TEST_FOO`.
    println!("cargo:rerun-if-changed=build.rs"); // Inform Cargo to rerun the build script if build.rs is changed.
    println!("cargo:rerun-if-env-changed=TEST_FOO"); // Inform Cargo to rerun the build script if TEST_FOO is changed.

    // Use the current timestamp to set the environment variable `TEST_FOO`.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    std::env::set_var("TEST_FOO", timestamp.to_string());

    // In tests8, enable the "pass" feature.
    println!("cargo:rerun-if-changed=build.rs"); // Inform Cargo to rerun the build script if build.rs is changed.
    let your_command = "pass";
    println!("cargo:{}", your_command);
}
