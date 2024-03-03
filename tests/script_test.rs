use std::error::Error;

use emerald::script::ScriptEngine;

mod common;

#[test]
fn test_script() -> Result<(), Box<dyn Error>> {
    let emerald = emerald::open(test_emerald!())?;
    let mut scripts = ScriptEngine::new();
    let inst = scripts.new_instance(&emerald)?;
    inst.execute(
        r#"
        for entry in emerald.fs.walk("") do
            print(entry.metadata.is_shard)
            print(entry.path:tostring())
        end
    "#,
    )?;

    Ok(())
}
