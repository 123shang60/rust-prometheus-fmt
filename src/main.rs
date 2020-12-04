use prometheus_exporter_base::prelude::*;
use psutil::memory::VirtualMemory;
use psutil::*;

fn main() {
    let swap_memory: VirtualMemory = memory::virtual_memory().unwrap();
    println!("{:?}", swap_memory);

    let mut pc = PrometheusMetric::build()
        .with_name("memory_use")
        .with_metric_type(MetricType::Gauge)
        .with_help("show memory use")
        .build();

    pc.render_and_append_instance(
        &PrometheusInstance::new()
            .with_label("type", "total")
            .with_value(swap_memory.total() / 1024 / 1024 / 1024),
    );

    pc.render_and_append_instance(
        &PrometheusInstance::new()
            .with_label("type", "free")
            .with_value(swap_memory.free() / 1024 / 1024 / 1024),
    );

    println!("{:?}", pc.render());
    println!("Hello, world!");
}
