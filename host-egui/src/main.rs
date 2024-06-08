fn main() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 440.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "wasm audio host egui example",
        native_options,
        Box::new(|cc| Box::new(host_egui::EguiApp::new(cc))),
    )
    .unwrap();

    Ok(())
}
