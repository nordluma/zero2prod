use zero2prod::{
    configuration::get_configurations,
    issue_delivery_worker::run_worker_until_stopped,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configurations().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone())
        .await?
        .run_until_stopped();

    let worker = run_worker_until_stopped(configuration);

    tokio::select! {
        _ = application => {},
        _ = worker => {},
    };

    Ok(())
}
