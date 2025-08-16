use aws_cdk_lib::{App, StageTrait};

pub fn synth_app(app: App) -> Result<(), Box<dyn std::error::Error>> {
    let cloud_assembly = app.synth(None);

    let stacks = cloud_assembly.get_stacks();

    for stack in stacks {
        let template_json = stack.get_template();
        
        let json_pretty = serde_json::to_string_pretty(&template_json)?;

        std::fs::create_dir("cdk.out")?;

        let filename = format!("cdk.out/{}-template.json", stack.get_stack_name());

        std::fs::write(&filename, json_pretty)?;
    }

    Ok(())
}
