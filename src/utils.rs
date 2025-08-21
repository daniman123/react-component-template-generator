use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
};

pub fn dir_check() -> bool {
    let src_components_dir_name = "src/components";
    let path = Path::new(src_components_dir_name);

    if path.is_dir() {
        println!("Found directory: {}", src_components_dir_name);
        true
    } else {
        println!(
            "'components' directory does not exist wihin 'src' directory: {}",
            src_components_dir_name
        );
        false
    }
}

pub struct CLIArgs {
    pub category_dir_name: String,
    pub component_name_dir_name: String,
    pub component_name: String,
}

pub fn parse_cli() -> CLIArgs {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    // Check if the expected number of arguments (2 in this case) is provided
    if args.len() != 3 {
        eprintln!("Error: invalid args used\n");
        eprintln!("Usage: <main_dir_name> <base_dir_name>");
        std::process::exit(1);
    }

    // Extract values from command-line arguments
    let category_dir_name = &args[1];
    let component_name_dir_name = &args[2];
    let component_name = set_component_name(component_name_dir_name.to_string());

    CLIArgs {
        category_dir_name: category_dir_name.to_string(),
        component_name_dir_name: component_name_dir_name.to_string(),
        component_name,
    }
}

pub fn set_file_extensions(base_dir: PathBuf) -> [PathBuf; 3] {
    let mut file_name_path = base_dir.clone();
    file_name_path.set_extension("tsx");
    let mut file_stories_name_path = base_dir.clone();
    file_stories_name_path.set_extension("stories.tsx");
    let mut file_mocks_name_path = base_dir.clone();
    file_mocks_name_path.set_extension("mocks.ts");

    [file_name_path, file_stories_name_path, file_mocks_name_path]
}

pub fn set_component_name(args: String) -> String {
    let chars = &mut args.chars();

    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

pub fn make_dynamic_dir_root(
    category_dir_name: String,
    component_name_dir_name: String,
) -> PathBuf {
    let current_dir = env::current_dir().expect("❌ Failed to get current working directory");
    println!("📂 Current directory: {}", current_dir.display());

    // Build the path without manual string concatenation
    let dynamic_dir = current_dir
        .join("src")
        .join("components")
        .join(category_dir_name)
        .join(component_name_dir_name);

    // Create the directory if it doesn't exist
    fs::create_dir_all(&dynamic_dir).expect("Error creating directory");
    dynamic_dir
}

pub fn construct_template(base_template: &str) -> Vec<String> {
    let base_template_tsx = format!(
        r#"export interface I{base_template} {{
exampleProp:string,
}}

const {base_template} = ({{exampleProp}}:I{base_template}) => {{
  return <div className="{base_template}-container">{{exampleProp}}</div>;
}};

export default {base_template};"#
    );

    let mock_ts = format!(
        r#"import {{ I{base_template} }} from './{base_template}';

const base: I{base_template} = {{
exampleProp: 'Hello world!',
}};
  
export const mock{base_template}Props = {{
base,
}};"#
    );

    let storybook_tsx = format!(
        r#"import type {{ Meta, StoryObj }} from "@storybook/nextjs-vite";
import {base_template} from "./{base_template}";
import {{ mock{base_template}Props }} from "./{base_template}.mocks";

const meta = {{
  component: {base_template},
}} satisfies Meta<typeof {base_template}>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {{
  args: {{ ...mock{base_template}Props.base }},
}};
"#
    );

    vec![base_template_tsx, mock_ts, storybook_tsx]
}
