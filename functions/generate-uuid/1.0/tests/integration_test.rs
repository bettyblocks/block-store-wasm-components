mod bindings {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "main",
    });
}

use bindings::Main;
use wasmtime::component::{Component, HasSelf, Linker, ResourceTable};
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

impl bindings::betty_blocks::random_hex::random_hex::Host for ComponentState {
    fn generate_random_hex(&mut self, size: u32) -> String {
        "A".repeat(size as usize)
    }
}

fn load_component(wasm_filename: &str) -> (Engine, Component) {
    let engine = Engine::default();
    let component_path = format!("{}/{wasm_filename}", env!("CARGO_MANIFEST_DIR"));
    let component =
        Component::from_file(&engine, &component_path).expect("failed to load WASM component");
    (engine, component)
}

fn instantiate_with_mock(engine: &Engine, component: &Component) -> (Store<ComponentState>, Main) {
    let mut linker: Linker<ComponentState> = Linker::new(engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker).expect("failed to add WASI to linker");
    Main::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)
        .expect("failed to add imports to linker");

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
fn exported_uuid_with_random_hex_is_valid() {
    let (engine, component) = load_component("generate_uuid.wasm");
    let (mut store, instance) = instantiate_with_mock(&engine, &component);

    let interface = instance.betty_blocks_generate_uuid_generate_uuid();
    let result = interface
        .call_generate_uuid(&mut store)
        .expect("failed to call generate-uuid");

    // Expected format: <uuid>-<random-hex>
    let parts: Vec<&str> = result.rsplitn(2, '-').collect();
    let random_hex_part = parts[0];
    let uuid_part = parts[1];

    assert_eq!(random_hex_part, "AAAAAAAA");
    assert_eq!(uuid_part.len(), 36);
}
