use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, SizedSample, SupportedOutputConfigs, SupportedStreamConfig,
};
use wasmtime::{
    component::{Component, Linker, Val},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

const CODE: &str = "~osc1: saw ~pitch
~osc2: squ ~pitch

~env: ~seq >> envperc 0.001 0.1 >> mul 1.0

~seq: speed 2.0 >> seq 60 _60 _60 60
>> mul 0.30

~pitch: ~seq >> mul 261.3

~t1: mix ~osc.. >> lpf 300.0 0.33 >> mul ~env
>> mul 1.5

o: mix ~t.. >> mul 1 >> plate 0.2

~t2: speed 4.0 >> seq _ 60 >> bd 0.2 >> mul 0.9

~t3: speed 4.0 >> seq 60 61 63 62 >> hh 0.02 >> mul 0.05";

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
    let mut wasm_config = Config::default();
    wasm_config.wasm_component_model(true);
    let engine = Engine::new(&wasm_config)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).expect("Failed to link command world");

    let wasi_view = ServerWasiView::new();
    let mut store = Store::new(&engine, wasi_view);

    let bytes = std::fs::read("techno.wasm")?;
    let component = Component::new(&engine, bytes)?;
    let instance = linker.instantiate(&mut store, &component)?;
    let func = instance
        .get_func(&mut store, "set-code")
        .expect("greet export not found");
    func.call(&mut store, &[Val::String(CODE.to_string())], &mut [])?;
    func.post_return(&mut store)?;

    let sample_rate = config.sample_rate.0 as f32;
    let func = instance
        .get_func(&mut store, "set-sr")
        .expect("greet export not found");
    func.call(&mut store, &[Val::Float32(sample_rate)], &mut [])?;
    func.post_return(&mut store)?;
    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let length = data.len() / channels;
            let max_time = 1.0 / sample_rate * length as f32;
            let start = std::time::Instant::now();

            let func = instance
                .get_func(&mut store, "process")
                .expect("greet export not found");

            let input = [Val::List(vec![Val::Float32(0.0); length])];
            let mut result = [Val::List(vec![Val::Float32(0.0); length])];

            func.call(&mut store, &input, &mut result).unwrap();
            func.post_return(&mut store).unwrap();

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

            println!("perc: {}%", perc * 100.0);
        },
        err_fn,
        None,
    )?;
    stream.play()?;
    std::thread::park();
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
