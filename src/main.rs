mod helpers;
mod my_full_stack_stack;

use my_full_stack_stack::MyFullStackStack;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = aws_cdk_lib::App::new(None);

    let _my_stack = MyFullStackStack::new(&app);

    helpers::synth_app(app)?;

    Ok(())
}
