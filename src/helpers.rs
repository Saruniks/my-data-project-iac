use aws_cdk_lib::App;

pub fn synth_app(app: App) -> Result<(), Box<dyn std::error::Error>> {
    let cloud_assembly = app.synth(None);

    let stacks = cloud_assembly.stacks();

    for stack in stacks {
        let template_json = stack.template_json();
        
        let json_pretty = serde_json::to_string_pretty(&template_json)?;

        std::fs::create_dir("cdk.out")?;

        let filename = format!("cdk.out/{}-template.json", stack.stack_name());

        std::fs::write(&filename, json_pretty)?;
    }

    Ok(())
}
