mod utils;

use crate::utils::{
    CLIArgs, construct_template, dir_check, make_dynamic_dir_root, parse_cli, set_file_extensions,
};

use std::{fs::File, io::Write};

fn main() {
    let does_dir_exist = dir_check();
    if does_dir_exist {
        let CLIArgs {
            category_dir_name,
            component_name,
            component_name_dir_name,
        } = parse_cli();

        let dynamic_dir_root = make_dynamic_dir_root(category_dir_name, component_name_dir_name);
        let base_dir = dynamic_dir_root.join(&component_name);

        let file_names = set_file_extensions(base_dir);

        for (index, file_content) in construct_template(&component_name).iter().enumerate() {
            // Create the file
            let file_name = file_names[index].as_os_str();

            let mut file = File::create(file_name).expect("Cannot create file");

            // // Write the content to the file
            file.write_all(file_content.as_bytes())
                .expect("Cannot write content to file");
            println!(
                "File \"{}\" created successfully.",
                file_names[index].display()
            );
        }
    }
}
