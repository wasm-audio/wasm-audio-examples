use std::{sync::Arc, thread};

use parking_lot::Mutex;
use wasmtime::{
    component::{Instance, Val},
    Store,
};

use crate::audio::*;

use cpal::{
    traits::{DeviceTrait, HostTrait},
    SupportedStreamConfig,
};

pub struct EguiApp {
    dropped_files: Vec<egui::DroppedFile>,
    instances: Arc<Mutex<Vec<(Store<ServerWasiView>, Instance)>>>,
    infos: Arc<Mutex<Vec<(Vec<ParamInfo>, String)>>>,
}

pub struct ParamInfo {
    name: String,
    min: f32,
    max: f32,
    value: f32,
}

impl EguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let instances = Arc::new(Mutex::new(vec![]));
        let infos = Arc::new(Mutex::new(vec![]));

        let instances_clone = Arc::clone(&instances);
        // let infos_clone = infos.clone();

        thread::spawn(move || {
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
                cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), instances_clone),
                cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), instances_clone),
                cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), instances_clone),
                cpal::SampleFormat::U8 => run::<u8>(&device, &config.into(), instances_clone),
                cpal::SampleFormat::I8 => run::<i8>(&device, &config.into(), instances_clone),
                cpal::SampleFormat::I32 => run::<i32>(&device, &config.into(), instances_clone),
                cpal::SampleFormat::U32 => run::<u32>(&device, &config.into(), instances_clone),
                cpal::SampleFormat::F64 => run::<f64>(&device, &config.into(), instances_clone),
                cpal::SampleFormat::U64 => run::<u64>(&device, &config.into(), instances_clone),
                cpal::SampleFormat::I64 => run::<i64>(&device, &config.into(), instances_clone),
                sample_format => panic!("Unsupported sample format '{sample_format}'"),
            }
            .unwrap()
        });

        Self {
            dropped_files: Default::default(),
            instances,
            infos,
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    if let Some(path) = &file.path {
                        let s = path.display().to_string();
                        let file_name_with_extension =
                            path.file_name().unwrap().to_str().unwrap().to_string();
                        let (mut store, instance) = match load_instance(&s) {
                            Ok((s, i)) => (s, i),
                            Err(e) => {
                                eprintln!("Failed to load instance: {:?}", e);
                                continue;
                            }
                        };

                        let info_func = match instance.get_func(&mut store, "get-params") {
                            Some(f) => f,
                            None => {
                                eprintln!("get-params function not found");
                                continue;
                            }
                        };

                        let mut result = [Val::List(vec![Val::Record(vec![])])];

                        if let Err(e) = info_func.call(&mut store, &[], &mut result) {
                            eprintln!("Failed to call get-params: {:?}", e);
                            continue;
                        }

                        if let Err(e) = info_func.post_return(&mut store) {
                            eprintln!("Failed to post return for get-params: {:?}", e);
                            continue;
                        }

                        match result[0].clone() {
                            Val::List(param_infos) => {
                                let mut info = vec![];
                                for param_info in param_infos {
                                    match param_info {
                                        Val::Record(params) => {
                                            let name = get_val_string(params[0].1.clone());
                                            let min = get_val_f32(params[1].1.clone());
                                            let max = get_val_f32(params[2].1.clone());
                                            let default = get_val_f32(params[3].1.clone());
                                            info.push(ParamInfo {
                                                name,
                                                min,
                                                max,
                                                value: default,
                                            });
                                        }
                                        _ => panic!("unexpected param info"),
                                    }
                                }
                                self.infos.lock().push((info, file_name_with_extension));
                            }
                            _ => panic!("unexpected return value"),
                        };

                        self.instances.lock().push((store, instance));
                    } else {
                        eprintln!("No path for file: {:?}", file);
                        continue;
                    }
                }
                self.dropped_files.extend_from_slice(&i.raw.dropped_files);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Warning! Lower the volume before playing audio.");
            ui.label("Drag and drop wasm audio plugin here:");
            if !self.infos.lock().is_empty() {
                for (i, info) in self.infos.lock().iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.label(&info.1);
                        ui.separator();
                        for info in info.0.iter_mut() {
                            ui.label(&info.name);
                            let response =
                                ui.add(egui::Slider::new(&mut info.value, info.min..=info.max));
                            if response.changed() {
                                // Lock the mutex and get the store and instance separately
                                let mut instances = self.instances.lock();
                                let (ref mut store, instance) = (*instances).get_mut(i).unwrap();

                                let set_param_func = match instance.get_func(&mut (*store), "set") {
                                    Some(f) => f,
                                    None => {
                                        panic!("set function not found");
                                        // continue;
                                    }
                                };

                                let input =
                                    [Val::String(info.name.clone()), Val::Float32(info.value)];
                                if let Err(e) = set_param_func.call(&mut (*store), &input, &mut [])
                                {
                                    eprintln!("Failed to call set function: {:?}", e);
                                    // continue;
                                }

                                if let Err(e) = set_param_func.post_return(&mut (*store)) {
                                    eprintln!("Failed to post return for set function: {:?}", e);
                                    // continue;
                                }
                            }
                        }
                    });
                }
            }
        });
    }
}

fn get_val_string(v: Val) -> String {
    match v {
        Val::String(s) => s,
        _ => panic!("unexpected value"),
    }
}

fn get_val_f32(v: Val) -> f32 {
    match v {
        Val::Float32(f) => f,
        _ => panic!("unexpected value"),
    }
}
