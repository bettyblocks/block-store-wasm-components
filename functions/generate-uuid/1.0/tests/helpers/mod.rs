pub mod bindings {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "main",
    });
}

use wasmtime::component::{Component, ComponentNamedList, Lift, Linker, Lower, ResourceTable};
use wasmtime::{Engine, Result, Store, StoreContextMut};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

pub struct ComponentState {
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

/// Builder for configuring mocks before instantiation.
pub struct ComponentHarness {
    engine: Engine,
    component: Component,
    linker: Linker<ComponentState>,
}

impl ComponentHarness {
    pub fn new(wasm_filename: &str) -> Self {
        let engine = Engine::default();
        let component_path = format!("{}/{wasm_filename}", env!("CARGO_MANIFEST_DIR"));
        let component = Component::from_file(&engine, &component_path)
            .expect("failed to load WASM component");

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)
            .expect("failed to add WASI to linker");

        ComponentHarness {
            engine,
            component,
            linker,
        }
    }

    pub fn mock<Parameters, Return>(
        &mut self,
        interface: &str,
        function: &str,
        handler: impl Fn(StoreContextMut<'_, ComponentState>, Parameters) -> Result<Return>
            + Send
            + Sync
            + 'static,
    ) -> &mut Self
    where
        Parameters: ComponentNamedList + Lift + 'static,
        Return: ComponentNamedList + Lower + 'static,
    {
        self.linker
            .instance(interface)
            .expect("failed to get linker instance")
            .func_wrap(function, handler)
            .expect("failed to register mock function");
        self
    }

    pub fn instantiate(self) -> InstantiatedComponent {
        let state = ComponentState {
            wasi_context: WasiCtxBuilder::new().build(),
            resource_table: ResourceTable::new(),
        };
        let mut store = Store::new(&self.engine, state);
        let instance = self
            .linker
            .instantiate(&mut store, &self.component)
            .expect("failed to instantiate component");
        let main = bindings::Main::new(&mut store, &instance)
            .expect("failed to create typed component wrapper");

        InstantiatedComponent { store, main }
    }
}

/// A ready-to-use component instance with typed export access.
pub struct InstantiatedComponent {
    pub store: Store<ComponentState>,
    pub main: bindings::Main,
}
