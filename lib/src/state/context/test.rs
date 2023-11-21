use crate::{
    plugin::{InterpFnFactory, Plugin, TagTypeHandler},
    types::Signature,
    Result, DEFAULT_CONTEXT,
};

#[test]
fn register_interp_plugin_succeeds() -> Result<()> {
    let context = DEFAULT_CONTEXT.register_plugins(&[&TEST_INTERP_PLUGIN]);
    if let Ok(ref ctx) = context {
        if ctx.0.interp_factory == TEST_INTERP {
            return Ok(());
        }
    } else {
        return Err(context.err().unwrap());
    }
    Err("Failed to register plugin")
}

static TEST_INTERP: InterpFnFactory = |_, _, _| panic!("This function should never run!!!");
static TEST_INTERP_PLUGIN: Plugin = Plugin::create_interpolation_plugin(&TEST_INTERP);
