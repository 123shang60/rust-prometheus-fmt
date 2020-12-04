use actix_web::{web, App, HttpServer, Responder};
use prometheus_exporter_base::prelude::*;
use psutil::memory::VirtualMemory;
use psutil::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().route("/metrics", web::get().to(metrics)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
        .unwrap();
    Ok(())
}

async fn metrics() -> impl Responder {
    let swap_memory: VirtualMemory = memory::virtual_memory().unwrap();

    let mut pc = PrometheusMetric::build()
        .with_name("memory_use")
        .with_metric_type(MetricType::Gauge)
        .with_help("show memory use")
        .build();

    pc.render_and_append_instance(
        &PrometheusInstance::new()
            .with_label("type", "total")
            .with_value(swap_memory.total()),
    );

    pc.render_and_append_instance(
        &PrometheusInstance::new()
            .with_label("type", "free")
            .with_value(swap_memory.free()),
    );
    pc.render()
}
