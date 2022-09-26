use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Layer};

fn main() {
    let filter = "unrelated=TRACE".parse::<EnvFilter>().unwrap();
    let layer = tracing_subscriber::fmt::layer().with_filter(filter);

    // commenting the next line makes the bug go away.
    let (layer, _handle) = tracing_subscriber::reload::Layer::new(layer);

    let subscriber = tracing_subscriber::registry();
    let subscriber = subscriber.with(layer);
    tracing::subscriber::set_global_default(subscriber).expect("could not set a global subscriber");
    tracing::debug!("{:?}", { eprintln!("you shall not print") });
}
