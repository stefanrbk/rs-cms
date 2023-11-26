use once_cell::sync::Lazy;

use crate::{
    plugin::{
        tag, FormatterInFactory, FormatterOutFactory, InterpFnFactory, Plugin, TagDescriptor,
        TagTypeHandler,
    },
    state::{Intent, Tag},
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

#[test]
fn register_tag_type_plugin_succeeds() -> Result<()> {
    let context = DEFAULT_CONTEXT.register_plugins(&[&TEST_INTERP_PLUGIN]);
    if let Ok(ref ctx) = context {
        let test = ctx.0.tag_types.last().unwrap();
        if test.sig == TEST_TAG_TYPE[0].sig && test.read == TEST_TAG_TYPE[0].read {
            return Ok(());
        }
    } else {
        return Err(context.err().unwrap());
    }
    Err("Failed to register plugin")
}

static TEST_TAG_TYPE: &[TagTypeHandler] = &[TagTypeHandler {
    sig: Signature::from_str(b"BUTT"),
    read: |_, _, _, _| panic!("This function should never run!!!"),
}];
static TEST_TAG_TYPE_PLUGIN: Plugin = Plugin::create_tag_type_plugin(&TEST_TAG_TYPE);

#[test]
fn register_mpe_type_plugin_succeeds() -> Result<()> {
    let context = DEFAULT_CONTEXT.register_plugins(&[&TEST_INTERP_PLUGIN]);
    if let Ok(ref ctx) = context {
        let test = ctx.0.tag_types.last().unwrap();
        if test.sig == TEST_TAG_TYPE[0].sig && test.read == TEST_TAG_TYPE[0].read {
            return Ok(());
        }
    } else {
        return Err(context.err().unwrap());
    }
    Err("Failed to register plugin")
}

static TEST_MPE_TYPE: &[TagTypeHandler] = &[TagTypeHandler {
    sig: Signature::from_str(b"BUTT"),
    read: |_, _, _, _| panic!("This function should never run!!!"),
}];
static TEST_MPE_TYPE_PLUGIN: Plugin = Plugin::create_mpe_type_plugin(&TEST_MPE_TYPE);

#[test]
fn register_tag_plugin_succeeds() -> Result<()> {
    let context = DEFAULT_CONTEXT.register_plugins(&[&TEST_TAG_PLUGIN]);
    if let Ok(ref ctx) = context {
        let test = ctx.0.tags.last().unwrap();
        if test == TEST_TAG[0] {
            return Ok(());
        }
    } else {
        return Err(context.err().unwrap());
    }
    Err("Failed to register plugin")
}

static TEST_TAG: &[&Tag] = &[&Tag {
    sig: Signature::from_str(b"BUTT"),
    desc: &tag::TagDescriptor::<[Signature; 1]> {
        elem_count: 2,
        decide_type: None,
        supported_types: [Signature::from_str(b"BUTT")],
    },
}];
static TEST_TAG_PLUGIN: Plugin = Plugin::create_tag_plugin(&TEST_TAG);

#[test]
fn register_formatter_plugin_succeeds() -> Result<()> {
    let context = DEFAULT_CONTEXT.register_plugins(&[&TEST_TAG_PLUGIN]);
    if let Ok(ref ctx) = context {
        let test = ctx.0.tags.last().unwrap();
        if test == TEST_TAG[0] {
            return Ok(());
        }
    } else {
        return Err(context.err().unwrap());
    }
    Err("Failed to register plugin")
}

static TEST_FORMATTER_IN: FormatterInFactory = |_, _| panic!("This function should never run!!!");
static TEST_FORMATTER_OUT: FormatterOutFactory = |_, _| panic!("This function should never run!!!");
static TEST_FORMATTER_PLUGIN: Plugin =
    Plugin::create_formatter_plugin(&(&TEST_FORMATTER_IN, &TEST_FORMATTER_OUT));

#[test]
fn register_rendering_intent_plugin_succeeds() -> Result<()> {
    let context = DEFAULT_CONTEXT.register_plugins(&[&TEST_RENDERING_INTENT_PLUGIN]);
    if let Ok(ref ctx) = context {
        let test = ctx.0.intents.last().unwrap();
        if test == &TEST_RENDERING_INTENTS[0] {
            return Ok(());
        }
    } else {
        return Err(context.err().unwrap());
    }
    Err("Failed to register plugin")
}

static TEST_RENDERING_INTENTS: &[Intent] = &[Intent {
    value: 0,
    desc: "Butts have cheeks",
    r#fn: |_, _, _, _, _, _, _| panic!("This function should never run!!!"),
}];
static TEST_RENDERING_INTENT_PLUGIN: Plugin =
    Plugin::create_intents_plugin(&TEST_RENDERING_INTENTS);
