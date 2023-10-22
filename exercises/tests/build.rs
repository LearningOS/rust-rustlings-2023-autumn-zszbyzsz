// // ! This is the build script for both tests7 and tests8.
// // !
// // ! You should modify this file to make both exercises pass.

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

// #[warn(unused_imports)]
fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.-s

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Set up the environment variable "TEST_FOO" with a value.
    let your_command = format!("cargo:rustc-env=TEST_FOO={}", timestamp.to_string());
    // let your_command = format!("TEST_FOO={}", timestamp.to_string());
    println!("{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "cargo:rustc-cfg=feature=\"pass\"";
    println!("{}", your_command);
}








