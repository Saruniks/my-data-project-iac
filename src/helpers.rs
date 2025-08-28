use std::ops::Deref;

use aws_cdk_lib::{cx_api::CloudAssemblyTrait, App, StageTrait};

pub fn synth_app(app: App) -> Result<(), Box<dyn std::error::Error>> {
    let cloud_assembly = app.synth(None);

    let stacks = cloud_assembly.get_stacks();

    // TODO: Is there a way to avoid ambiguity in cases like these while generating Rust code?
    let assembly_dir = CloudAssemblyTrait::get_directory(cloud_assembly.deref());
    println!("Cloud assembly directory: {}", assembly_dir);

    let cdk_out = std::path::Path::new("cdk.out");
    if !cdk_out.exists() {
        std::fs::create_dir("cdk.out")?;
    }

    copy_directory_contents(&assembly_dir, cdk_out)?;

    Ok(())
}

fn copy_directory_contents(src: &str, dest: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if file_type.is_dir() {
            if !dest_path.exists() {
                std::fs::create_dir(&dest_path)?;
            }
            copy_directory_contents(src_path.to_str().unwrap(), &dest_path)?;
        } else {
            std::fs::copy(&src_path, &dest_path)?;
        }
    }
    Ok(())
}
