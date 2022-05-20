
use std::process::Command;
use dialoguer::{console::Term, Select,MultiSelect, theme::ColorfulTheme};
use std::io::{self, Write};

fn main() {
  let mut commit_message = String::new();
  println!("Ingrese mensaje del commit: ");
  io::stdin().read_line(&mut commit_message);

  let base_branch = Command::new("git").arg("rev-parse").arg("--abbrev-ref").arg("HEAD").output().unwrap();
  let base_branch = String::from_utf8(base_branch.stdout).unwrap();
  let base_branch = base_branch.trim();

  let optiones = Command::new("git").arg("branch").output().unwrap();
  let options_full_str= String::from_utf8(optiones.stdout).unwrap();
  let options_arr: Vec<&str> = options_full_str.split(" ").collect::<Vec<&str>>();
  let mut real_options = Vec::new();
  for branch in options_arr{
    if branch.len()>1{
      real_options.push(branch.trim())
    }
  }
  let mut selected:usize =0 ;

  println!("Selecciones la rama destino: ");
  while true{
    selected = Select::with_theme(&ColorfulTheme::default())
        .items(&real_options)
        .default(0)
        .interact_on(&Term::stderr()).unwrap();
    break
  }
  let selected_branch = real_options.iter().nth(selected).unwrap();

  println!("Comenzando merge a {}", selected_branch);
  let add_command = Command::new("git").arg("add").arg(".").status();
  let commit_command = Command::new("git").arg("commit").arg("-m").arg(&commit_message).status();
  let push_command = Command::new("git").arg("push").status();
  
  let checkout_target_command = Command::new("git").arg("checkout").arg(selected_branch).status();
  let merge_command = Command::new("git").arg("merge").arg("origin").arg(base_branch).status();
  let add_command = Command::new("git").arg("add").arg(".").status();
  let commit_command = Command::new("git").arg("commit").arg("-m").arg(commit_message).status();
  let push_command = Command::new("git").arg("push").status();
  
  let checkout_base_command = Command::new("git").arg("checkout").arg(base_branch).status();
}
