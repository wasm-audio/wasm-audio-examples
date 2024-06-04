use wasmtime::{
    component::{Component, Linker, Val},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

fn main() -> anyhow::Result<()> {
    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).expect("Failed to link command world");

    let wasi_view = ServerWasiView::new();
    let mut store = Store::new(&engine, wasi_view);

    let bytes = std::fs::read("../guest/target/wasm32-wasi/release/wasm_cm3.wasm")?;
    let component = Component::new(&engine, bytes)?;
    let instance = linker.instantiate(&mut store, &component)?;
    let func = instance
        .get_func(&mut store, "set-freq")
        .expect("greet export not found");
    func.call(&mut store, &[Val::Float32(1000.0)], &mut [])?;
    func.post_return(&mut store)?;
    for _ in 0..10 {
        let func = instance
            .get_func(&mut store, "process")
            .expect("greet export not found");
        let input = [Val::List(vec![Val::Float32(1.0), Val::Float32(2.0)])];
        let mut result = [Val::List(vec![Val::Float32(0.0), Val::Float32(0.0)])];
        func.call(&mut store, &input, &mut result)?;
        println!("{:?}", result);
        func.post_return(&mut store)?;
    }
    Ok(())
}

struct ServerWasiView {
    table: ResourceTable,
    ctx: WasiCtx,
}

impl ServerWasiView {
    fn new() -> Self {
        let table = ResourceTable::new();
        let ctx = WasiCtxBuilder::new().inherit_stdio().build();

        Self { table, ctx }
    }
}

impl WasiView for ServerWasiView {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
