mod bindings {
    wasmtime::component::bindgen!({ path: "wit", world: "main" });
}

wasmtime_testing_helper::setup!(bindings);
