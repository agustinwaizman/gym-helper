use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::fmt::format::{FormatEvent, FormatFields, Writer};
use tracing::{Subscriber, Event};
use tracing_subscriber::registry::LookupSpan;
use tracing_appender::rolling::RollingFileAppender;
use std::fmt::Result;

pub fn init() -> WorkerGuard {
    // Configura un appender de archivo que rota diariamente
    let file_appender = RollingFileAppender::new(
        tracing_appender::rolling::Rotation::DAILY,
        "logs",
        "app.log"
    );
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Configura el formato personalizado
    let format = CustomFormat;

    // Configura el suscriptor con el formato y el writer
    let subscriber = fmt()
        .with_writer(non_blocking)
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .event_format(format)
        .finish();

    // Establece el suscriptor global
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    guard
}

struct CustomFormat;

impl<S, N> FormatEvent<S, N> for CustomFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &fmt::FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> Result {
        let meta = event.metadata();

        // Timestamp con formato personalizado
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f");

        // Nivel del log
        let level = meta.level().to_string();

        // Target y línea (si está disponible)
        let target = meta.target();
        let line = meta.line().unwrap_or(0);

        // Escribimos el encabezado
        write!(
            writer,
            "[GH_1][{}][{}][{}:{}] ",
            now, level, target, line
        )?;

        // Escribimos los campos del evento usando una referencia mutable
        ctx.format_fields(writer.by_ref(), event)?;

        // Añadimos una nueva línea
        writeln!(writer)?;

        Ok(())
    }
}
