use clap::ArgMatches;
use std::fs::{set_permissions, File, Permissions};
use std::io::prelude::*;
use std::os::unix::prelude::PermissionsExt;

pub fn script_gen(location_command: &str, hook: &str, hook_config_file: &str) -> String {
  format!(
    "
#!/bin/sh
$ {}
# Hook created by precommit
{} run {} {}
",
    hook, location_command, hook, hook_config_file
  )
}

pub fn execute(matches: &ArgMatches) -> Result<(), ()> {
  let location = "./target/debug/precommit";
  let hook = "pre-commit";
  let hook_config_file = matches
    .values_of("hook_config_file")
    .unwrap()
    .next()
    .unwrap();

  let template = script_gen(location, hook, hook_config_file);
  let git_hook_file = format!(".git/hooks/{}", hook);

  let mut file = File::create(&git_hook_file).expect("failed to create file");
  file
    .write_all(&template.into_bytes())
    .expect("failed to write to file");

  set_permissions(&git_hook_file, Permissions::from_mode(0o777)).expect("Failed to chmod: {}");

  Ok(())
}
