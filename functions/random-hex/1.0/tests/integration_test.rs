mod bindings {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "main",
    });
}

use bindings::Main;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

struct ComponentState {
    wasi_context: WasiCtx,
    resource_table: ResourceTable,
}

impl WasiView for ComponentState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_context,
            table: &mut self.resource_table,
        }
    }
}

fn load_component(wasm_filename: &str) -> (Engine, Component) {
    let engine = Engine::default();
    let component_path = format!("{}/{wasm_filename}", env!("CARGO_MANIFEST_DIR"));
    let component =
        Component::from_file(&engine, &component_path).expect("failed to load WASM component");
    (engine, component)
}

fn instantiate(engine: &Engine, component: &Component) -> (Store<ComponentState>, Main) {
    let mut linker: Linker<ComponentState> = Linker::new(engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker).expect("failed to add WASI to linker");

    let state = ComponentState {
        wasi_context: WasiCtxBuilder::new().build(),
        resource_table: ResourceTable::new(),
    };
    let mut store = Store::new(engine, state);
    let instance =
        Main::instantiate(&mut store, component, &linker).expect("failed to instantiate component");
    (store, instance)
}

#[test]
fn generate_random_hex_returns_correct_length() {
    let (engine, component) = load_component("random_hex.wasm");
    let (mut store, instance) = instantiate(&engine, &component);

    let interface = instance.betty_blocks_random_hex_random_hex();
    let result = interface
        .call_generate_random_hex(&mut store, 16)
        .expect("failed to call generate-random-hex");

    assert_eq!(result.len(), 16);
}

#[test]
fn generate_random_hex_produces_valid_hex() {
    let (engine, component) = load_component("random_hex.wasm");
    let (mut store, instance) = instantiate(&engine, &component);

    let interface = instance.betty_blocks_random_hex_random_hex();
    let result = interface
        .call_generate_random_hex(&mut store, 32)
        .expect("failed to call generate-random-hex");

    assert!(
        u128::from_str_radix(&result, 16).is_ok(),
        "result '{result}' is not valid hex"
    );
}

#[test]
fn generate_random_hex_with_zero_size_returns_empty() {
    let (engine, component) = load_component("random_hex.wasm");
    let (mut store, instance) = instantiate(&engine, &component);

    let interface = instance.betty_blocks_random_hex_random_hex();
    let result = interface
        .call_generate_random_hex(&mut store, 0)
        .expect("failed to call generate-random-hex");

    assert!(result.is_empty());
}
