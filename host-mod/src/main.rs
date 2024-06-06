use parking_lot::Mutex;
use std::sync::Arc;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, SizedSample, SupportedStreamConfig,
};
use wasmtime::{
    component::{Component, Linker, Val},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use hashbrown::HashMap;

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("no output device available");
    let config = device
        .default_output_config()
        .expect("no default output config available");

    let config = SupportedStreamConfig::new(
        2,
        config.sample_rate(),
        config.buffer_size().clone(),
        config.sample_format(),
    );

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into()),
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into()),
        // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into()),
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into()),
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
        // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into()),
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into()),
        // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into()),
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into()),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into()),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }?;

    Ok(())
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let processors = Arc::new(Mutex::new(vec![]));
    let processors_clone = Arc::clone(&processors);

    let sample_rate = config.sample_rate.0 as f32;
    let sin_processor = load_wasm(
        "./sin.wasm",
        HashMap::from_iter([
            ("freq", Val::Float32(880.0)),
            ("sample_rate", Val::Float32(sample_rate)),
        ]),
    )?;
    processors.lock().push(sin_processor);

    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let length = data.len() / channels;
            let max_time = 1.0 / sample_rate * length as f32;
            let start = std::time::Instant::now();

            let mut result = [Val::List(vec![Val::Float32(0.0); length])];

            for processor in processors.lock().iter_mut() {
                let mut processor = processor.lock();
                result[0] = processor(result[0].clone());
            }

            let value = match &result[0] {
                Val::List(val) => val,
                _ => panic!("unexpected value {:?}", result[0]),
            };

            let result = value
                .iter()
                .map(|val| match val {
                    Val::Float32(val) => *val,
                    _ => panic!("unexpected value {:?}", val),
                })
                .collect::<Vec<f32>>();

            for (i, val) in result.iter().enumerate() {
                let val = T::from_sample(*val);
                data[i * channels] = val;
                data[i * channels + 1] = val;
            }

            let elapsed = start.elapsed().as_secs_f32();
            let perc = elapsed / max_time;
            if perc > 0.7 {
                println!("perc: {}%", perc * 100.0);
            }
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    // after 1 second, push mul processor

    std::thread::sleep(std::time::Duration::from_millis(2000));

    for i in 0..50 {
        println!("pushing mul processor no {}", i);
        let mul_processor = load_wasm(
            "./mul.wasm",
            HashMap::from_iter([("factor", Val::Float32(0.95))]),
        )?;
        processors_clone.lock().push(mul_processor);
    }

    // it can hanle 50 processors

    // make it forever
    std::thread::park();
    Ok(())
}

fn load_wasm(
    name: &str,
    args: HashMap<&str, Val>,
) -> anyhow::Result<Arc<Mutex<dyn FnMut(Val) -> Val + Send + Sync>>> {
    let mut wasm_config = Config::default();
    wasm_config.wasm_component_model(true);
    let engine = Engine::new(&wasm_config)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).expect("Failed to link command world");

    let wasi_view = ServerWasiView::new();
    let mut store = Store::new(&engine, wasi_view);

    let bytes = std::fs::read(name)?;
    let component = Component::new(&engine, bytes)?;
    let instance = linker.instantiate(&mut store, &component)?;

    for (arg_name, arg_val) in args {
        let func = instance
            .get_func(&mut store, "set")
            .expect("func export not found");
        let arg_key = Val::String(arg_name.to_string());
        func.call(&mut store, &[arg_key, arg_val], &mut [])?;
        func.post_return(&mut store)?;
    }

    let func = instance
        .get_func(&mut store, "process")
        .expect("func export not found");

    let store = Arc::new(Mutex::new(store));
    let func = Arc::new(Mutex::new(func));

    let processor = {
        let store = Arc::clone(&store);
        let func = Arc::clone(&func);

        move |input: Val| -> Val {
            let mut store = store.lock();
            let func = func.lock();
            let mut result = [Val::List(vec![Val::Float32(0.0); 1024])];
            func.call(&mut *store, &[input], &mut result).unwrap();
            func.post_return(&mut *store).unwrap();
            result[0].clone()
        }
    };

    Ok(Arc::new(Mutex::new(processor)))
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
