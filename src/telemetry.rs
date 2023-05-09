use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry, fmt::MakeWriter};

pub fn trace_subscriber<Sink>(name: String, env_filter: String, sink: Sink) -> impl Subscriber + Sync + Send 
where
Sink: for<'a> MakeWriter<'a> + Sync + Send + 'static
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(formatting_layer)
        .with(JsonStorageLayer)
        .with(env_filter)
}

pub fn init_tracing(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Log tracer to be initialised");
    tracing::subscriber::set_global_default(subscriber)
        .expect("Subscriber must be able to set for tracing");
}
