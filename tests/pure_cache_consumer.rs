use anyhow::Result;
use wasi_cap_std_sync::{ambient_authority, Dir, WasiCtxBuilder};
use wasi_common::WasiCtx;
use wasmtime::{Config, Engine, Linker, Module, Store};

use wasm_mock::pure::wit_cache;
pub use wit_cache::add_to_linker;

use wasm_mock::pure::PureCache;

const RUST_MODULE: &str = "./crates/simple-consumer/target/wasm32-wasi/release/simple-consumer.wasm";

#[test]
fn test_simple_consumer() -> Result<()> {
    let mut config = Config::new();
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_multi_memory(true);
    config.wasm_module_linking(true);

    let engine = Engine::new(&config)?;
    let module = Module::from_file(&engine, RUST_MODULE)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut Context| &mut cx.wasi)?;

    let rc = PureCache::new();
    wit_cache::add_to_linker(&mut linker, |ctx| -> &mut PureCache {
        ctx.runtime_data.as_mut().unwrap()
    })?;

    let ctx = Context {
        runtime_data: Some(rc),
        wasi: default_wasi(),
    };

    let mut store = Store::new(&engine, ctx);
    let instance = linker.instantiate(&mut store, &module)?;

    let start = instance.get_func(&mut store, "_start").unwrap();
    start.call(&mut store, &[], &mut [])?;

    Ok(())
}

struct Context {
    pub wasi: WasiCtx,
    pub runtime_data: Option<PureCache>,
}

fn default_wasi() -> WasiCtx {
    let mut ctx = WasiCtxBuilder::new().inherit_stdio();
    ctx = ctx
        .preopened_dir(
            Dir::open_ambient_dir("./target", ambient_authority()).unwrap(),
            "cache",
        )
        .unwrap();

    ctx.build()
}