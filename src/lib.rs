// © Copyright 2026 Helsing GmbH. All rights reserved.
//! A `tracing-subscriber` layer for beautifully printing spans and events to the
//! terminal.
//!
//! This crate provides a `ConsoleLayer` that can be used with `tracing` to format
//! and display log messages in a hierarchical and easy-to-read format. It is
//! designed to make command line applications more pleasant by providing clear
//! visual cues for different log levels, tracking the duration of spans, and
//! indicating success or failure states.
//!
//! ## Features
//!
//! - **Pretty Printing**: Spans and events are displayed with icons, colors, and
//!   indentation to create a clear visual hierarchy.
//! - **Automatic `Result` Handling**: The `console!` macro automatically logs
//!   the outcome of a `Result`, indicating success or failure.
//! - **Span Timings**: The layer automatically tracks and displays the execution
//!   time of each span.
//! - **Error Propagation**: Errors inside spans are propagated to parent spans,
//!   marking them as failed.
//!
//! ## API Documentation
//!
//! [Click here!](https://docs.rs/tracing-scribe/latest/tracing_scribe/)
//!
//! ## Usage
//!
//! To use `ConsoleLayer`, add it to your `tracing` subscriber registry. The
//! `console!` macro can then be used to log events with different status levels.
//!
//! ### Example
//!
//! Here is a complete example demonstrating a successful execution with nested
//! spans and `#[tracing::instrument]`.
//!
//! ```rust
//! use tracing_scribe::console;
//! use tracing::info_span;
//! use tracing_subscriber::prelude::*;
//! use tracing_subscriber::registry::Registry;
//!
//! #[tracing::instrument]
//! fn outer_task() {
//!     inner_task();
//! }
//!
//! #[tracing::instrument]
//! fn inner_task() {
//!     console!(notice, "Generating unicorns");
//!     std::thread::sleep(std::time::Duration::from_millis(100));
//!     console!(info, "Unicorns generation completed");
//! }
//!
//! let subscriber = Registry::default().with(tracing_scribe::ConsoleLayer::default());
//! tracing::subscriber::with_default(subscriber, || {
//!     let _span = info_span!("main").entered();
//!     outer_task();
//! });
//! ```
//!
//! This will produce the following output:
//!
//! ```text
//! ╭─ ⚙ main
//! │  ╭─ ⚙ outer_task
//! │  │  ╭─ ⚙ inner_task
//! │  │  │  ├─ ! Generating unicorns
//! │  │  │  ├─ ✔ Unicorns generation completed
//! │  │  ╰─ ✔ inner_task ⏱ 100.2ms
//! │  ╰─ ✔ outer_task ⏱ 100.3ms
//! ╰─ ✔ main succeeded ⏱ 100.3ms
//! ```
//!
//! ### Error Handling
//!
//! The `ConsoleLayer` also handles errors gracefully. When a span instrumented
//! with `#[tracing::instrument(err)]` returns an error, the span and its parents
//! are marked as failed.
//!
//! ```rust
//! use tracing_scribe::console;
//! use tracing::info_span;
//! use tracing_subscriber::prelude::*;
//! use tracing_subscriber::registry::Registry;
//! use std::error::Error;
//! use std::fmt;
//!
//! #[derive(Debug)]
//! struct MyError;
//!
//! impl Error for MyError {}
//!
//! impl fmt::Display for MyError {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         write!(f, "Something went wrong")
//!     }
//! }
//!
//! #[tracing::instrument(err)]
//! fn failing_task() -> Result<(), MyError> {
//!     console!(error, "This task is about to fail");
//!     Err(MyError)
//! }
//!
//! let subscriber = Registry::default().with(tracing_scribe::ConsoleLayer::default());
//! tracing::subscriber::with_default(subscriber, || {
//!     let _span = info_span!("main").entered();
//!     failing_task().ok();
//! });
//! ```
//!
//! This will produce the following output:
//!
//! ```text
//! ╭─ ⚙ main
//! │  ╭─ ⚙ failing_task
//! │  │  ├─ ⨯ This task is about to fail
//! │  │  ├─ ⨯ Something went wrong
//! │  ╰─ ⨯ failing_task ⏱ 63.8µs
//! ╰─ ⨯ main failed ⏱ 111.4µs
//! ```
//!
//! ### Asynchronous tasks
//!
//! Asynchronous tasks are handled their own way by tracing. To make sure they end up with the
//! right indentation, you might have to wrap them with a tracing instrumentation call.
//!
//! ```no_run
//! use tracing_scribe::console;
//! use tracing::Instrument;
//!
//! #[tracing::instrument]
//! async fn outer_task() {
//!     tokio::spawn(
//!         inner_task().instrument(tracing::Span::current())
//!     );
//! }
//!
//! #[tracing::instrument]
//! async fn inner_task() {
//!     console!(notice, "Generating async unicorns");
//!     tokio::time::sleep(std::time::Duration::from_millis(100)).await;
//!     console!(info, "Async unicorns generation completed");
//! }
//! ```
//!
//! This will produce the following output:
//!
//! ```text
//! ╭─ ⚙ main
//! │  ╭─ ⚙ outer_task
//! │  │  ╭─ ⚙ inner_task
//! │  │  │  ├─ ! Generating async unicorns
//! │  │  │  ├─ ✔ Async unicorns generation completed
//! │  │  ╰─ ✔ inner_task ⏱ 100.2ms
//! │  ╰─ ✔ outer_task ⏱ 100.3ms
//! ╰─ ✔ main succeeded ⏱ 100.3ms
//! ```
//!
//! ### Attaching Extra Information
//!
//! The `extra` field can be used to attach additional context to a span, which
//! will be displayed alongside the span name.
//!
//! ```rust
//! use tracing_scribe::console;
//! use tracing::info_span;
//! use tracing_subscriber::prelude::*;
//! use tracing_subscriber::registry::Registry;
//!
//! #[tracing::instrument(fields(extra = "some extra info"))]
//! fn task_with_extra() {
//!     console!(info, "This task has extra information");
//! }
//!
//! let subscriber = Registry::default().with(tracing_scribe::ConsoleLayer::default());
//! tracing::subscriber::with_default(subscriber, || {
//!     let _span = info_span!("main").entered();
//!     task_with_extra();
//! });
//! ```
//!
//! This will produce the following output:
//!
//! ```text
//! ╭─ ⚙ main
//! │  ╭─ ⚙ task_with_extra some extra info
//! │  │  ├─ ✔ This task has extra information
//! │  ╰─ ✔ task_with_extra some extra info ⏱ 2.8µs
//! ╰─ ✔ main succeeded ⏱ 23.3µs
//! ```
//! ### Custom Status
//!
//! The `with_status` method allows you to customize the icons and colors used
//! for the different statuses.
//!
//! ```rust
//! use tracing_scribe::{console, ConsoleLayer, StatusFactory, Status};
//! use tracing::info_span;
//! use tracing_subscriber::prelude::*;
//! use tracing_subscriber::registry::Registry;
//! use owo_colors::OwoColorize;
//! use std::io::{self, Write};
//!
//! #[derive(Clone, Copy)]
//! pub struct CustomStatusFactory;
//!
//! impl StatusFactory<CustomStatus> for CustomStatusFactory {
//!     fn group(&self) -> CustomStatus {
//!         CustomStatus::Group
//!     }
//!     fn success(&self) -> CustomStatus {
//!         CustomStatus::Success
//!     }
//!     fn failure(&self) -> CustomStatus {
//!         CustomStatus::Failure
//!     }
//! }
//!
//! #[derive(Debug, Clone, Copy)]
//! pub enum CustomStatus {
//!     Success,
//!     Failure,
//!     Group,
//!     Pass,
//! }
//!
//! impl Status for CustomStatus {
//!     fn from_str(s: &str) -> Self {
//!         match s {
//!             "success" => Self::Success,
//!             _ => Self::Failure,
//!         }
//!     }
//!
//!     fn write_icon<W: Write>(&self, mut out: W, colors: &tracing_scribe::ColorScheme) -> io::Result<()> {
//!         match self {
//!             Self::Success => write!(out, "{}", colors.success.style("●")),
//!             Self::Failure => write!(out, "{}", colors.error.style("●")),
//!             Self::Group => write!(out, "{}", colors.group.style("●")),
//!             Self::Pass => write!(out, ""),
//!         }
//!     }
//!
//!     fn is_passthrough(&self) -> bool {
//!         matches!(self, Self::Pass)
//!     }
//! }
//!
//! let subscriber = Registry::default().with(
//!     ConsoleLayer::default().with_status(CustomStatusFactory)
//! );
//! tracing::subscriber::with_default(subscriber, || {
//!     let _span = info_span!("main").entered();
//!     console!(warn, "This is a custom status");
//! });
//! ```
//!
//! This will produce the following output:
//!
//! ```text
//! ╭─ ● main
//! │  ├─ ● This is a custom status
//! ╰─ ● main succeeded
//! ```
use std::io::{self, Write};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use owo_colors::{OwoColorize, Style};
use tracing::field::Visit;
use tracing::{Event, Id};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::registry::LookupSpan;

/// A color scheme for customizing the appearance of console output.
///
/// This struct allows you to customize the colors used for different elements
/// in the console output, including status icons, tree structure, messages,
/// and various text elements.
///
/// ## Example
///
/// ```rust
/// use tracing_scribe::{ConsoleLayer, ColorScheme};
/// use owo_colors::{Style, colors};
/// use tracing_subscriber::prelude::*;
/// use tracing_subscriber::registry::Registry;
///
/// let colors = ColorScheme::default()
///     .with_success_color(colors::BrightGreen)
///     .with_error_color(colors::BrightRed)
///     .with_tree_color(colors::Cyan);
///
/// let layer = ConsoleLayer::default().with_colors(colors);
/// let subscriber = Registry::default().with(layer);
/// ```
#[derive(Clone, Debug)]
pub struct ColorScheme {
    /// Style for success icons and messages
    pub success: Style,
    /// Style for error/failure icons and messages
    pub error: Style,
    /// Style for warning icons and messages
    pub warning: Style,
    /// Style for notice icons and messages
    pub notice: Style,
    /// Style for group/span icons
    pub group: Style,
    /// Style for tree structure characters (pipes, boxes)
    pub tree: Style,
    /// Style for the "extra" field in spans
    pub extra: Style,
    /// Style for the "succeeded" message
    pub succeeded: Style,
    /// Style for the "failed" message
    pub failed: Style,
    /// Style for duration/clock text
    pub duration: Style,
}

impl Default for ColorScheme {
    /// Creates a default color scheme matching the original hardcoded colors.
    fn default() -> Self {
        Self {
            success: Style::new().green(),
            error: Style::new().red(),
            warning: Style::new().yellow(),
            notice: Style::new().purple(),
            group: Style::new().dimmed(),
            tree: Style::new().blue(),
            extra: Style::new().cyan(),
            succeeded: Style::new().green().bold(),
            failed: Style::new().red().bold(),
            duration: Style::new().dimmed(),
        }
    }
}

impl ColorScheme {
    /// Creates a new color scheme with all colors set to their defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the color for success icons and messages.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tracing_scribe::ColorScheme;
    /// use owo_colors::colors::BrightGreen;
    ///
    /// let colors = ColorScheme::new().with_success_color(BrightGreen);
    /// ```
    pub fn with_success_color<C: owo_colors::Color>(mut self, _color: C) -> Self {
        self.success = Style::new().fg::<C>();
        self
    }

    /// Sets the style for success icons and messages.
    pub fn with_success_style(mut self, style: Style) -> Self {
        self.success = style;
        self
    }

    /// Sets the color for error/failure icons and messages.
    pub fn with_error_color<C: owo_colors::Color>(mut self, _color: C) -> Self {
        self.error = Style::new().fg::<C>();
        self
    }

    /// Sets the style for error/failure icons and messages.
    pub fn with_error_style(mut self, style: Style) -> Self {
        self.error = style;
        self
    }

    /// Sets the color for warning icons and messages.
    pub fn with_warning_color<C: owo_colors::Color>(mut self, _color: C) -> Self {
        self.warning = Style::new().fg::<C>();
        self
    }

    /// Sets the style for warning icons and messages.
    pub fn with_warning_style(mut self, style: Style) -> Self {
        self.warning = style;
        self
    }

    /// Sets the color for notice icons and messages.
    pub fn with_notice_color<C: owo_colors::Color>(mut self, _color: C) -> Self {
        self.notice = Style::new().fg::<C>();
        self
    }

    /// Sets the style for notice icons and messages.
    pub fn with_notice_style(mut self, style: Style) -> Self {
        self.notice = style;
        self
    }

    /// Sets the color for group/span icons.
    pub fn with_group_color<C: owo_colors::Color>(mut self, _color: C) -> Self {
        self.group = Style::new().fg::<C>();
        self
    }

    /// Sets the style for group/span icons.
    pub fn with_group_style(mut self, style: Style) -> Self {
        self.group = style;
        self
    }

    /// Sets the color for tree structure characters.
    pub fn with_tree_color<C: owo_colors::Color>(mut self, _color: C) -> Self {
        self.tree = Style::new().fg::<C>();
        self
    }

    /// Sets the style for tree structure characters.
    pub fn with_tree_style(mut self, style: Style) -> Self {
        self.tree = style;
        self
    }

    /// Sets the color for the "extra" field in spans.
    pub fn with_extra_color<C: owo_colors::Color>(mut self, _color: C) -> Self {
        self.extra = Style::new().fg::<C>();
        self
    }

    /// Sets the style for the "extra" field in spans.
    pub fn with_extra_style(mut self, style: Style) -> Self {
        self.extra = style;
        self
    }

    /// Sets the color for the "succeeded" message.
    pub fn with_succeeded_color<C: owo_colors::Color>(mut self, _color: C) -> Self {
        self.succeeded = Style::new().fg::<C>().bold();
        self
    }

    /// Sets the style for the "succeeded" message.
    pub fn with_succeeded_style(mut self, style: Style) -> Self {
        self.succeeded = style;
        self
    }

    /// Sets the color for the "failed" message.
    pub fn with_failed_color<C: owo_colors::Color>(mut self, _color: C) -> Self {
        self.failed = Style::new().fg::<C>().bold();
        self
    }

    /// Sets the style for the "failed" message.
    pub fn with_failed_style(mut self, style: Style) -> Self {
        self.failed = style;
        self
    }

    /// Sets the color for duration/clock text.
    pub fn with_duration_color<C: owo_colors::Color>(mut self, _color: C) -> Self {
        self.duration = Style::new().fg::<C>();
        self
    }

    /// Sets the style for duration/clock text.
    pub fn with_duration_style(mut self, style: Style) -> Self {
        self.duration = style;
        self
    }
}

/// A trait that defines the behavior of a status.
///
/// This trait can be implemented to customize the icons and colors used in the
/// console output.
pub trait Status: Send + Sync {
    /// Creates a `Status` from a string.
    ///
    /// This is used by the `console!` macro to create a status from the level
    /// specified in the macro invocation.
    fn from_str(s: &str) -> Self;

    /// Writes the icon for the status to the given writer.
    fn write_icon<W: Write>(&self, out: W, colors: &ColorScheme) -> io::Result<()>;

    /// Specify which status code can be used for passthrough printing, allowing to remove
    /// icons and styles when using the `pass` argument to the `console!` macro.
    fn is_passthrough(&self) -> bool;
}

/// A trait that is responsible for creating statuses.
///
/// This trait can be implemented to customize the statuses used in the console
/// output.
pub trait StatusFactory<S: Status>: Send + Sync {
    /// Creates a status for a group.
    fn group(&self) -> S;

    /// Creates a status for a success.
    fn success(&self) -> S;

    /// Creates a status for a failure.
    fn failure(&self) -> S;
}

/// The default status implementation.
#[derive(Debug, Clone, Copy)]
pub enum DefaultStatus {
    Notice,
    Success,
    Failure,
    Warning,
    Group,
    Pass,
}

impl Status for DefaultStatus {
    fn from_str(s: &str) -> Self {
        match s {
            "notice" => Self::Notice,
            "success" => Self::Success,
            "warn" => Self::Warning,
            "error" => Self::Failure,
            "pass" => Self::Pass,
            _ => Self::Success,
        }
    }

    fn write_icon<W: Write>(&self, mut out: W, colors: &ColorScheme) -> io::Result<()> {
        match self {
            Self::Notice => write!(out, "{}", colors.notice.style("!")),
            Self::Success => write!(out, "{}", colors.success.style("✔")),
            Self::Failure => write!(out, "{}", colors.error.style("⨯")),
            Self::Warning => write!(out, "{}", colors.warning.style("⚠")),
            Self::Group => write!(out, "{}", colors.group.style("⚙")),
            Self::Pass => write!(out, ""),
        }
    }

    fn is_passthrough(&self) -> bool {
        matches!(self, Self::Pass)
    }
}

/// The default status factory.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultStatusFactory;

impl StatusFactory<DefaultStatus> for DefaultStatusFactory {
    fn group(&self) -> DefaultStatus {
        DefaultStatus::Group
    }

    fn success(&self) -> DefaultStatus {
        DefaultStatus::Success
    }

    fn failure(&self) -> DefaultStatus {
        DefaultStatus::Failure
    }
}

/// Format a duration with 1 decimal place, choosing the most readable unit.
///
/// Tier thresholds are set so that rounding to 1 decimal place never produces
/// `"1000.0ms"` or `"1000.0µs"` — values that would round up are promoted to
/// the next unit instead. Sub-microsecond durations are displayed as whole
/// nanoseconds.
fn format_duration(d: Duration) -> String {
    let nanos = d.as_nanos();
    // 999_950_000ns rounds to 1000.0ms at 1 decimal place, so promote to seconds.
    if nanos >= 999_950_000 {
        format!("{:.1}s", d.as_secs_f64())
    } else if nanos >= 999_950 {
        // 999_950ns rounds to 1000.0µs at 1 decimal place, so promote to milliseconds.
        format!("{:.1}ms", nanos as f64 / 1_000_000.0)
    } else if nanos >= 1_000 {
        format!("{:.1}µs", nanos as f64 / 1_000.0)
    } else {
        format!("{nanos}ns")
    }
}

/// A `tracing-subscriber` layer that formats and prints events to the console.
///
/// This layer is responsible for the hierarchical and colorful output, including
/// span timings, status icons, and error propagation. It keeps track of the
/// state of each span and formats messages accordingly.
///
/// # Panics
///
/// A subscriber can hold only one `ConsoleLayer`. With two of them, creating a
/// span panics, because both layers keep their per-span state in the same slot
/// of the span's extensions.
#[derive(Debug)]
pub struct ConsoleLayer<W, F = DefaultStatusFactory, S = DefaultStatus> {
    make_writer: W,
    status_factory: F,
    colors: Arc<ColorScheme>,
    _status: PhantomData<S>,
}

impl Default for ConsoleLayer<fn() -> io::Stderr> {
    /// Creates a new `ConsoleLayer` that writes to `io::stderr`.
    ///
    /// This is the default constructor and the most common way to create a
    /// `ConsoleLayer`.
    fn default() -> Self {
        Self {
            make_writer: io::stderr,
            status_factory: DefaultStatusFactory,
            colors: Arc::new(ColorScheme::default()),
            _status: PhantomData,
        }
    }
}

impl<W> ConsoleLayer<W> {
    /// Creates a new `ConsoleLayer` that writes to a custom writer.
    ///
    /// This allows for capturing output in tests or redirecting it to a file
    /// or other sink.
    pub fn with_writer(make_writer: W) -> Self {
        Self {
            make_writer,
            status_factory: DefaultStatusFactory,
            colors: Arc::new(ColorScheme::default()),
            _status: PhantomData,
        }
    }
}

impl<W, F, S> ConsoleLayer<W, F, S> {
    // The following constants define the characters used to draw the tree
    // structure in the console output.

    /// The string printed at the beginning of a root span.
    const HEADER_START: &str = "╭─";
    /// The string printed at the end of a root span.
    const FOOTER_START: &str = "╰─";
    /// The string printed at the beginning of a nested span.
    const GROUP_START: &str = "╭─";
    /// The string printed at the end of a nested span.
    const GROUP_END: &str = "╰─";
    /// The string printed before an event message.
    const ITEM_START: &str = "├─";
    /// The string used to draw a continuous vertical line.
    const PIPE: &str = "│  ";
    /// The string used to draw a vertical line with a prefix.
    const PIPE_PREFIX: &str = "│  ";
    /// The icon used to indicate a time measurement.
    const CLOCK: &str = "⏱";

    /// Creates a new `ConsoleLayer` with a custom status factory.
    pub fn with_status<CustomF, CustomS>(
        self,
        status_factory: CustomF,
    ) -> ConsoleLayer<W, CustomF, CustomS>
    where
        CustomF: StatusFactory<CustomS>,
        CustomS: Status,
    {
        ConsoleLayer {
            make_writer: self.make_writer,
            status_factory,
            colors: self.colors,
            _status: PhantomData,
        }
    }

    /// Sets a custom color scheme for the console output.
    ///
    /// This allows you to customize the colors used for different elements
    /// in the output, such as status icons, tree structure, and messages.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tracing_scribe::{ConsoleLayer, ColorScheme};
    /// use owo_colors::colors;
    ///
    /// let colors = ColorScheme::default()
    ///     .with_success_color(colors::BrightGreen)
    ///     .with_error_color(colors::BrightRed);
    ///
    /// let layer = ConsoleLayer::default().with_colors(colors);
    /// ```
    pub fn with_colors(mut self, colors: ColorScheme) -> Self {
        self.colors = Arc::new(colors);
        self
    }

    /// Prints a single line to the console with the proper formatting.
    ///
    /// This internal method is responsible for constructing the final output string
    /// for a single event or span boundary. It combines the indentation, status
    /// icon, message, and optional duration into a single formatted line.
    fn print_line<St: Status>(
        &self,
        depth: usize,
        start_symbol: &str,
        status: St,
        message: &str,
        duration: Option<Duration>,
        bold: bool,
    ) where
        W: for<'writer> MakeWriter<'writer>,
    {
        let mut out = self.make_writer.make_writer();

        if depth > 0 {
            // Every other level has one initial pipe, followed by (depth - 1) prefixes.
            write!(out, "{}", self.colors.tree.style(Self::PIPE)).ok();
            for _ in 1..depth {
                write!(out, "{}", self.colors.tree.style(Self::PIPE_PREFIX)).ok();
            }
        }

        write!(out, "{}", self.colors.tree.style(start_symbol)).ok();
        write!(out, " ").ok();
        status.write_icon(&mut out, &self.colors).ok();
        if bold {
            write!(out, " {}", message.bold()).ok();
        } else {
            write!(out, " {}", message).ok();
        }
        if let Some(d) = duration {
            write!(
                out,
                " {}",
                self.colors
                    .duration
                    .style(format!("{} {} ", Self::CLOCK, format_duration(d)))
            )
            .ok();
        }
        writeln!(out).ok();
    }
}

#[derive(Debug)]
struct ConsoleSpanInfo {
    extra: Option<String>,
    has_failed: AtomicBool,
    started: OnceLock<Instant>,
}

impl<S, W, F, St> Layer<S> for ConsoleLayer<W, F, St>
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
    W: for<'writer> MakeWriter<'writer> + 'static + Send + Sync,
    F: StatusFactory<St> + 'static,
    St: Status + 'static,
{
    /// Called when a new span is created.
    ///
    /// This method is called by the `tracing` framework whenever a new span
    /// is created. We use this to extract any relevant fields from the span
    /// and attach them to the span's extensions for later use. This allows
    /// us to access fields like `extra` when the span is entered or closed.
    fn on_new_span(&self, attrs: &tracing::span::Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        if let Some(span) = ctx.span(id) {
            let mut visitor = FieldVisitor::default();
            attrs.record(&mut visitor);

            // Span extensions are shared by all layers, so a second ConsoleLayer
            // finds the info the first one stored.
            assert!(
                span.extensions().get::<ConsoleSpanInfo>().is_none(),
                "ConsoleLayer was added more than once to the same subscriber"
            );
            span.extensions_mut().insert(ConsoleSpanInfo {
                extra: visitor.extra,
                has_failed: AtomicBool::new(false),
                started: OnceLock::new(),
            });
        }
    }

    /// Called when a span is entered.
    ///
    /// This is triggered when the `tracing` instrumentation context is entered.
    /// We record the `Instant` of entry to calculate the span's duration later.
    /// It then prints the formatted header for the span, indicating that a new
    /// block of execution has started.
    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        if let Some(span) = ctx.span(id) {
            let extensions = span.extensions();
            let info = extensions.get::<ConsoleSpanInfo>().unwrap();
            // Only the first entry prints the header, even if threads race here.
            if info.started.set(Instant::now()).is_err() {
                return;
            }

            let metadata = span.metadata();
            let depth = span.scope().from_root().count() - 1;
            let start_symbol = if depth == 0 {
                Self::HEADER_START
            } else {
                Self::GROUP_START
            };
            let message = if let Some(extra) = &info.extra {
                format!("{} {}", metadata.name(), self.colors.extra.style(extra))
            } else {
                metadata.name().to_string()
            };
            self.print_line(
                depth,
                start_symbol,
                self.status_factory.group(),
                &message,
                None,
                true,
            );
        }
    }

    /// Called when an event is dispatched.
    ///
    /// This method is the core of the event logging logic. It receives all
    /// `tracing` events and decides how to display them.
    ///
    /// - It uses the `FieldVisitor` to extract relevant data like the message,
    ///   status, and error details.
    /// - It checks if the event represents a failure (i.e., has a `Level::ERROR`).
    /// - If an event is a failure, it propagates the failed state to all parent spans.
    /// - It formats and prints the event message, but only for events created
    ///   with the `console!` macro or for events that represent an error. This
    ///   is to avoid printing standard `tracing` events that are not intended
    ///   for this layer.
    fn on_event(&self, event: &Event, ctx: Context<'_, S>) {
        let mut visitor = FieldVisitor::default();
        event.record(&mut visitor);

        // An event is a failure if its level is ERROR.
        // This catches errors from `#[instrument(err)]` and `tracing::error!`.
        let is_failure = *event.metadata().level() == tracing::Level::ERROR;

        let mut message = visitor.message.clone();

        // For non-console errors (e.g. from `#[instrument(err)]` propagation),
        // check whether the innermost span was already marked as failed BEFORE
        // we propagate. If it was, a child span already printed this error —
        // we still propagate has_failed but skip printing the duplicate message.
        let was_already_failed = if is_failure && !visitor.console {
            ctx.event_scope(event)
                .and_then(|mut scope| scope.next())
                .and_then(|span| {
                    span.extensions()
                        .get::<ConsoleSpanInfo>()
                        .map(|info| info.has_failed.load(Ordering::SeqCst))
                })
                .unwrap_or(false)
        } else {
            false
        };

        if is_failure {
            // Mark all parent spans as failed
            if let Some(scope) = ctx.event_scope(event) {
                for span in scope.from_root() {
                    if let Some(info) = span.extensions().get::<ConsoleSpanInfo>() {
                        info.has_failed.store(true, Ordering::SeqCst);
                    }
                }
            }
            // Prepend error details to the message if available
            if let Some(error) = visitor.error {
                if message.is_empty() {
                    message = error;
                } else {
                    message = format!("{message}: {error}");
                }
            }
        }

        // Only print events that are explicitly for the console, or are
        // first-time errors. Duplicate errors from `#[instrument(err)]`
        // propagation are suppressed — the span close lines still show ⨯.
        if visitor.console || (is_failure && !was_already_failed) {
            let status = if visitor.console {
                St::from_str(&visitor.status)
            } else {
                // It must be a failure if we got here without `visitor.console` being true
                self.status_factory.failure()
            };

            let (start_symbol, bold) = if status.is_passthrough() {
                ("", false)
            } else {
                (Self::ITEM_START, true)
            };

            let depth = ctx.event_scope(event).map_or(0, |scope| scope.count());
            self.print_line(depth, start_symbol, status, &message, None, bold);
        }
    }

    /// Called when a span is closed.
    ///
    /// When a span's context is exited, this method is called. It calculates
    /// the total duration the span was active by comparing the current `Instant`
    /// with the one stored in `on_enter`.
    ///
    /// It then determines the final status of the span (success or failure) based
    /// on whether any errors were reported within it. Finally, it prints the
    /// formatted footer for the span, including its total duration and final status.
    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        if let Some(span) = ctx.span(&id) {
            let extensions = span.extensions();
            let info = extensions.get::<ConsoleSpanInfo>().unwrap();

            let duration = info.started.get().map(|start_time| start_time.elapsed());

            let metadata = span.metadata();
            let has_failed = info.has_failed.load(Ordering::SeqCst);
            let status = if has_failed {
                self.status_factory.failure()
            } else {
                self.status_factory.success()
            };

            let base_message = if let Some(extra) = &info.extra {
                format!("{} {}", metadata.name(), self.colors.extra.style(extra))
            } else {
                metadata.name().to_string()
            };

            let depth = span.scope().from_root().count() - 1;
            let (final_message, start_symbol) = if depth == 0 {
                let status_string = if has_failed {
                    self.colors.failed.style("failed").to_string()
                } else {
                    self.colors.succeeded.style("succeeded").to_string()
                };
                (
                    format!("{base_message} {status_string}"),
                    Self::FOOTER_START,
                )
            } else {
                (base_message, Self::GROUP_END)
            };
            self.print_line(depth, start_symbol, status, &final_message, duration, true);
        }
    }
}

/// A visitor to extract "status", "message", and other relevant fields from a
/// tracing event or span.
///
/// This struct implements the `tracing::field::Visit` trait, which allows it to
/// traverse the fields of an event or span and capture the values we care about.
/// An instance of `FieldVisitor` is created for each event and span to collect
/// the necessary data for formatting the console output.
#[derive(Default)]
struct FieldVisitor {
    status: String,
    message: String,
    console: bool,
    extra: Option<String>,
    error: Option<String>,
}

impl Visit for FieldVisitor {
    /// Captures string values for fields we are interested in.
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "message" => self.message = value.to_string(),
            "status" => self.status = value.to_string(),
            "extra" => self.extra = Some(value.to_string()),
            _ => {}
        }
    }

    /// Captures boolean values. This is used to detect if an event was
    /// created by the `console!` macro.
    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        if let "console" = field.name() {
            self.console = value
        }
    }

    /// Captures debug-formatted values, primarily used for extracting error
    /// information from the `error` field.
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "error" {
            self.error = Some(format!("{value:?}"));
        }
    }
}

#[doc(hidden)]
/// Internal helpers for the `console!` macro.
///
/// This module is not part of the public API and should not be used directly.
/// It contains a collection of traits and functions that provide the underlying
/// logic for the `console!` macro, particularly for handling `Result` types
/// and formatting success and failure messages.
///
/// The core of this module is the `ConsoleResult` trait, which allows the macro
/// to differentiate between `Result`s that return a `()` on success and those
/// that return a value implementing `Display`. This is what enables the macro
/// to either print just a message, or a message followed by the success value.
pub mod macros {
    use owo_colors::OwoColorize;
    use std::fmt::Display;

    /// A wrapper to help the compiler resolve trait implementations.
    ///
    /// This is part of the mechanism to allow the `console!` macro to handle both
    /// `Result<T, E>` where `T: Display` and `Result<(), E>`. By wrapping the successor
    /// value in `Displayable`, we can provide a distinct type for the `ConsoleResult`
    /// trait to implement.
    pub struct Displayable<T>(pub T);

    /// A trait to format the success message of a `console!` invocation.
    ///
    /// This trait is the key to allowing the `console!` macro to handle different
    /// success types in a `Result`. We provide two implementations: one for the unit
    /// type `()`, and a generic one for any type that implements `Display`.
    ///
    /// The `console!` macro calls `format_success` on the `Ok` value of a `Result`.
    /// Depending on which implementation is chosen, the message will either be
    /// printed as-is (for `()`) or with the value appended (for `T: Display`).
    pub trait ConsoleResult<T> {
        /// Formats the success message.
        fn format_success(self, colorize: impl Fn(String) -> String, message: String) -> String;
    }

    impl ConsoleResult<()> for () {
        fn format_success(self, _colorize: impl Fn(String) -> String, message: String) -> String {
            message
        }
    }

    impl<T: Display> ConsoleResult<Displayable<T>> for T {
        fn format_success(self, colorize: impl Fn(String) -> String, message: String) -> String {
            let this = self.to_string();
            if this.is_empty() {
                message
            } else {
                format!("{}: {}", message, colorize(self.to_string()))
            }
        }
    }

    /// Formats a failure message.
    ///
    /// This is a helper function that takes an error which implements `Display`
    /// and a message, and formats them into a standardized failure string with
    /// the error details colored in red.
    pub fn format_failure<E: Display>(err: &E, message: String) -> String {
        format!("{}: {}", message, err.to_string().red())
    }

    /// Colors a success value green, the default for the `console!` macro.
    ///
    /// The macro calls this through `$crate` so that callers do not need
    /// `owo_colors::OwoColorize` in scope.
    pub fn default_colorize(s: String) -> String {
        s.green().to_string()
    }
}

#[doc(hidden)]
/// An internal helper macro to handle the logic for a `Result`.
/// This is not intended to be called directly.
#[macro_export]
macro_rules! __console_impl_result {
    ($result:expr, $colorize:expr, $($arg:tt)+) => {
        match $result {
            Ok(result) => {
                use $crate::macros::ConsoleResult;
                let message = result.format_success($colorize, format!($($arg)+));
                tracing::event!(
                    tracing::Level::INFO,
                    console = true,
                    status = "success",
                    message = message
                );
                Ok(result)
            }
            Err(e) => {
                let error_message = $crate::macros::format_failure(&e, format!($($arg)+));
                tracing::event!(
                    tracing::Level::ERROR,
                    console = true,
                    status = "error",
                    message = error_message
                );
                Err(e)
            }
        }
    };
}

/// Creates a tracing event with the required fields for our `ConsoleLayer`.
///
/// This macro provides a  way to log events with different status levels and to handle
/// `Result` types automatically.
///
/// ## Usage
///
/// The macro has several forms depending on the desired outcome.
///
/// ### Simple Logging
///
/// You can log a simple message with a specific status. The supported statuses are
/// `notice`, `info`, `warn`, and `error`.
///
/// ```rust
/// # use tracing_scribe::console;
/// # fn main() {
/// console!(notice, "This is a notice");
/// console!(info, "This is an informational message");
/// console!(warn, "This is a warning");
/// console!(error, "This is an error");
/// # }
/// ```
///
/// Finally, you can completely passthrough colors and text styles by using the `pass` argument to
/// the macro.
///
/// ```rust
/// # use tracing_scribe::console;
/// # fn main() {
/// console!(pass, "This is a string with no style");
/// # }
/// ```
///
/// ### Handling `Result` Types
///
/// The macro can also be used to log the outcome of a `Result`. It will automatically
/// log a success or failure message based on the variant of the `Result`.
///
/// If the `Ok` variant of the `Result` contains a value that implements `Display`,
/// it will be appended to the message.
///
/// ```rust
/// # use tracing_scribe::console;
/// # fn main() {
/// let successful_result: Result<&str, &str> = Ok("everything is fine");
/// console!(successful_result, "A successful operation");
///
/// let failed_result: Result<&str, &str> = Err("something went wrong");
/// console!(failed_result, "A failed operation");
/// # }
/// ```
///
/// If the `Ok` variant is a unit type (`()`), no extra value is appended to the message.
///
/// ```rust
/// # use tracing_scribe::console;
/// # fn main() {
/// let successful_result: Result<(), &str> = Ok(());
/// console!(successful_result, "An operation with no output");
/// # }
/// ```
///
/// You can also customize the color of the success message output.
///
/// ```rust
/// # use tracing_scribe::console;
/// use owo_colors::OwoColorize;
/// # fn main() {
/// let successful_result: Result<&str, &str> = Ok("everything is fine");
/// console!(successful_result, |s: String| s.blue().to_string(), "A successful operation with custom colors");
/// # }
/// ```
#[macro_export]
macro_rules! console {
    (pass, $($arg:tt)+) => {
        tracing::event!(tracing::Level::INFO, console = true, status = "pass", message = &format!($($arg)+));
    };
    (notice, $($arg:tt)+) => {
        tracing::event!(tracing::Level::INFO, console = true, status = "notice", message = &format!($($arg)+));
    };
    (info, $($arg:tt)+) => {
        tracing::event!(tracing::Level::INFO, console = true, status = "success", message = &format!($($arg)+));
    };
    (warn, $($arg:tt)+) => {
        tracing::event!(tracing::Level::WARN, console = true, status = "warn", message = &format!($($arg)+));
    };
    (error, $($arg:tt)+) => {
        tracing::event!(tracing::Level::ERROR, console = true, status = "error", message = &format!($($arg)+));
    };
    // Before the `$color:expr` arm, which would take the format string as the color.
    ($result:expr, $fmt:literal $($arg:tt)*) => {
        $crate::__console_impl_result!($result, $crate::macros::default_colorize, $fmt $($arg)*)
    };
    ($result:expr, $color:expr, $($arg:tt)+) => {
        $crate::__console_impl_result!($result, $color, $($arg)+)
    };
    ($result:expr, $($arg:tt)+) => {
        $crate::__console_impl_result!($result, $crate::macros::default_colorize, $($arg)+)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::fmt::Display;
    use std::sync::{Arc, Mutex};
    use tracing_subscriber::Registry;
    use tracing_subscriber::fmt::MakeWriter;
    use tracing_subscriber::prelude::*;

    #[derive(Debug, Clone)]
    struct MockWriter {
        buf: Arc<Mutex<Vec<u8>>>,
    }

    impl Display for MockWriter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                String::from_utf8(self.buf.lock().unwrap().clone()).unwrap()
            )
        }
    }

    impl MockWriter {
        fn new() -> Self {
            Self {
                buf: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    impl<'a> MakeWriter<'a> for MockWriter {
        type Writer = Self;

        fn make_writer(&'a self) -> Self::Writer {
            self.clone()
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buf.lock().unwrap().write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.buf.lock().unwrap().flush()
        }
    }

    fn strip_non_deterministic(output: &str) -> String {
        // Strip ANSI escape codes
        let output = strip_ansi_escapes::strip_str(output);

        // Strip durations
        let duration_regex = Regex::new(r" ⏱ .*? ").unwrap();
        duration_regex.replace_all(&output, "").trim().to_string()
    }

    #[test]
    fn test_simple_span() {
        let writer = MockWriter::new();
        let subscriber = Registry::default().with(ConsoleLayer::with_writer(writer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            let _span = tracing::info_span!("simple").entered();
            console!(info, "It works!");
        });

        let output = strip_non_deterministic(&writer.to_string());
        insta::assert_snapshot!(output, @r###"
        ╭─ ⚙ simple
        │  ├─ ✔ It works!
        ╰─ ✔ simple succeeded
        "###);
    }

    #[test]
    fn test_other_layer_storing_instant() {
        // Span extensions are shared by all layers, so a plain `Instant` stored
        // by another layer must not hide the span header.
        struct Timing;
        impl<S: tracing::Subscriber + for<'a> LookupSpan<'a>> Layer<S> for Timing {
            fn on_new_span(&self, _: &tracing::span::Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
                ctx.span(id)
                    .unwrap()
                    .extensions_mut()
                    .insert(Instant::now());
            }
        }

        let writer = MockWriter::new();
        let subscriber = Registry::default()
            .with(Timing)
            .with(ConsoleLayer::with_writer(writer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            let _span = tracing::info_span!("simple").entered();
            console!(info, "It works!");
        });

        let output = strip_non_deterministic(&writer.to_string());
        insta::assert_snapshot!(output, @r###"
        ╭─ ⚙ simple
        │  ├─ ✔ It works!
        ╰─ ✔ simple succeeded
        "###);
    }

    #[test]
    #[should_panic(expected = "ConsoleLayer was added more than once to the same subscriber")]
    fn test_two_console_layers() {
        let subscriber = Registry::default()
            .with(ConsoleLayer::with_writer(MockWriter::new()))
            .with(ConsoleLayer::with_writer(MockWriter::new()));

        tracing::subscriber::with_default(subscriber, || {
            let _span = tracing::info_span!("simple").entered();
        });
    }

    #[test]
    fn test_nested_spans() {
        let writer = MockWriter::new();
        let subscriber = Registry::default().with(ConsoleLayer::with_writer(writer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            let _outer = tracing::info_span!("outer").entered();
            let _inner = tracing::info_span!("inner").entered();
            console!(notice, "Something is happening");
        });

        let output = strip_non_deterministic(&writer.to_string());
        insta::assert_snapshot!(output, @r###"
        ╭─ ⚙ outer
        │  ╭─ ⚙ inner
        │  │  ├─ ! Something is happening
        │  ╰─ ✔ inner
        ╰─ ✔ outer succeeded
        "###);
    }

    #[test]
    fn test_error_propagation() {
        let writer = MockWriter::new();
        let subscriber = Registry::default().with(ConsoleLayer::with_writer(writer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            #[tracing::instrument(name = "inner", err)]
            fn inner() -> Result<(), &'static str> {
                Err("An error occurred")
            }
            let _outer = tracing::info_span!("outer").entered();
            inner().ok()
        });

        let output = strip_non_deterministic(&writer.to_string());
        insta::assert_snapshot!(output, @r"
        ╭─ ⚙ outer
        │  ╭─ ⚙ inner
        │  │  ├─ ⨯ An error occurred
        │  ╰─ ⨯ inner
        ╰─ ⨯ outer failed
        ");
    }

    #[test]
    fn test_nested_instrument_err_deduplication() {
        let writer = MockWriter::new();
        let subscriber = Registry::default().with(ConsoleLayer::with_writer(writer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            #[tracing::instrument(name = "inner", err)]
            fn inner() -> Result<(), &'static str> {
                Err("something went wrong")
            }

            #[tracing::instrument(name = "middle", err)]
            fn middle() -> Result<(), &'static str> {
                inner()
            }

            #[tracing::instrument(name = "outer", err)]
            fn outer() -> Result<(), &'static str> {
                middle()
            }

            let _root = tracing::info_span!("root").entered();
            outer().ok();
        });

        let output = strip_non_deterministic(&writer.to_string());
        // The error message should appear only once (at the innermost span),
        // but all parent spans should still show the failure icon on close.
        insta::assert_snapshot!(output, @r"
        ╭─ ⚙ root
        │  ╭─ ⚙ outer
        │  │  ╭─ ⚙ middle
        │  │  │  ╭─ ⚙ inner
        │  │  │  │  ├─ ⨯ something went wrong
        │  │  │  ╰─ ⨯ inner
        │  │  ╰─ ⨯ middle
        │  ╰─ ⨯ outer
        ╰─ ⨯ root failed
        ");
    }

    /// Reproduces the "branch already exists" CI failure scenario.
    ///
    /// Without deduplication, the error message would appear 3 times (once at
    /// each `#[instrument(err)]` level). With deduplication it appears once.
    #[test]
    fn test_bump_branch_already_exists() {
        let writer = MockWriter::new();
        let subscriber = Registry::default().with(ConsoleLayer::with_writer(writer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            #[tracing::instrument(name = "run command git checkout -b bump/v0.15.4", err)]
            fn git_checkout_new_branch() -> Result<(), String> {
                console!(warn, "fatal: a branch named 'bump/v0.15.4' already exists");
                Err(
                    "error running command git checkout -b bump/v0.15.4, exit code: Some(128)"
                        .to_string(),
                )
            }

            #[tracing::instrument(name = "bump", err)]
            fn bump_inner() -> Result<(), String> {
                {
                    let _span = tracing::info_span!("configure git").entered();
                    {
                        let _cmd =
                            tracing::info_span!("run command git config user.name").entered();
                    }
                    {
                        let _cmd =
                            tracing::info_span!("run command git config user.email").entered();
                    }
                }
                {
                    let _span = tracing::info_span!("fetch and checkout").entered();
                    {
                        let _cmd =
                            tracing::info_span!("run command git fetch origin develop").entered();
                    }
                    {
                        let _cmd = tracing::info_span!("run command git checkout origin/develop")
                            .entered();
                    }
                }
                console!(info, "current version: 0.15.3");
                console!(info, "new version: 0.15.4");
                {
                    let _cmd = tracing::info_span!(
                        "run command git ls-remote --exit-code --heads origin bump/v0.15.4"
                    )
                    .entered();
                }
                git_checkout_new_branch()
            }

            #[tracing::instrument(name = "bump", err)]
            fn bump_outer() -> Result<(), String> {
                bump_inner()
            }

            let _cli = tracing::info_span!("cli").entered();
            console!(notice, "run id: 019c7593-1f68-784a-ade1-9a06a33a38be");
            bump_outer().ok();
            console!(notice, "run id: 019c7593-1f68-784a-ade1-9a06a33a38be");
        });

        let output = strip_non_deterministic(&writer.to_string());
        insta::assert_snapshot!(output, @r"
        ╭─ ⚙ cli
        │  ├─ ! run id: 019c7593-1f68-784a-ade1-9a06a33a38be
        │  ╭─ ⚙ bump
        │  │  ╭─ ⚙ bump
        │  │  │  ╭─ ⚙ configure git
        │  │  │  │  ╭─ ⚙ run command git config user.name
        │  │  │  │  ╰─ ✔ run command git config user.name
        │  │  │  │  ╭─ ⚙ run command git config user.email
        │  │  │  │  ╰─ ✔ run command git config user.email
        │  │  │  ╰─ ✔ configure git
        │  │  │  ╭─ ⚙ fetch and checkout
        │  │  │  │  ╭─ ⚙ run command git fetch origin develop
        │  │  │  │  ╰─ ✔ run command git fetch origin develop
        │  │  │  │  ╭─ ⚙ run command git checkout origin/develop
        │  │  │  │  ╰─ ✔ run command git checkout origin/develop
        │  │  │  ╰─ ✔ fetch and checkout
        │  │  │  ├─ ✔ current version: 0.15.3
        │  │  │  ├─ ✔ new version: 0.15.4
        │  │  │  ╭─ ⚙ run command git ls-remote --exit-code --heads origin bump/v0.15.4
        │  │  │  ╰─ ✔ run command git ls-remote --exit-code --heads origin bump/v0.15.4
        │  │  │  ╭─ ⚙ run command git checkout -b bump/v0.15.4
        │  │  │  │  ├─ ⚠ fatal: a branch named 'bump/v0.15.4' already exists
        │  │  │  │  ├─ ⨯ error running command git checkout -b bump/v0.15.4, exit code: Some(128)
        │  │  │  ╰─ ⨯ run command git checkout -b bump/v0.15.4
        │  │  ╰─ ⨯ bump
        │  ╰─ ⨯ bump
        │  ├─ ! run id: 019c7593-1f68-784a-ade1-9a06a33a38be
        ╰─ ⨯ cli failed
        ");
    }

    /// Reproduces the "push permission denied" CI failure scenario.
    ///
    /// Without deduplication, the long error message would appear 4 times (once
    /// at each `#[instrument(err)]` level). With deduplication it appears once.
    #[test]
    fn test_bump_push_permission_denied() {
        let writer = MockWriter::new();
        let subscriber = Registry::default().with(ConsoleLayer::with_writer(writer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            #[tracing::instrument(name = "run command git push -u origin HEAD", err)]
            fn git_push() -> Result<(), String> {
                console!(
                    warn,
                    "remote: You are not allowed to push code to this project."
                );
                console!(
                    warn,
                    "fatal: unable to access 'https://git.example.com/example/project.git/': The requested URL returned error: 403"
                );
                Err("You are not allowed to push code to this project.".to_string())
            }

            #[tracing::instrument(name = "commit and push", err)]
            fn commit_and_push() -> Result<(), String> {
                {
                    let _cmd =
                        tracing::info_span!("run command git add Cargo.toml Cargo.lock").entered();
                }
                {
                    let _cmd =
                        tracing::info_span!("run command git commit -m bump version to 0.15.4")
                            .entered();
                }
                git_push()
            }

            #[tracing::instrument(name = "bump", err)]
            fn bump_inner() -> Result<(), String> {
                {
                    let _span = tracing::info_span!("configure git").entered();
                }
                {
                    let _span = tracing::info_span!("fetch and checkout").entered();
                }
                console!(info, "current version: 0.15.3");
                console!(info, "new version: 0.15.4");
                {
                    let _cmd =
                        tracing::info_span!("run command git checkout -b bump/v0.15.4").entered();
                }
                {
                    let _span = tracing::info_span!("update cargo.toml").entered();
                    console!(info, "updated Cargo.toml to version 0.15.4");
                }
                {
                    let _span = tracing::info_span!("update cargo.lock").entered();
                    {
                        let _cmd =
                            tracing::info_span!("run command cargo update --workspace").entered();
                    }
                    console!(info, "updated Cargo.lock");
                }
                commit_and_push()
            }

            #[tracing::instrument(name = "bump", err)]
            fn bump_outer() -> Result<(), String> {
                bump_inner()
            }

            let _cli = tracing::info_span!("cli").entered();
            console!(notice, "run id: 019c75a8-e7a6-7bec-822a-e371c363d73e");
            bump_outer().ok();
            console!(notice, "run id: 019c75a8-e7a6-7bec-822a-e371c363d73e");
        });

        let output = strip_non_deterministic(&writer.to_string());
        insta::assert_snapshot!(output, @r"
        ╭─ ⚙ cli
        │  ├─ ! run id: 019c75a8-e7a6-7bec-822a-e371c363d73e
        │  ╭─ ⚙ bump
        │  │  ╭─ ⚙ bump
        │  │  │  ╭─ ⚙ configure git
        │  │  │  ╰─ ✔ configure git
        │  │  │  ╭─ ⚙ fetch and checkout
        │  │  │  ╰─ ✔ fetch and checkout
        │  │  │  ├─ ✔ current version: 0.15.3
        │  │  │  ├─ ✔ new version: 0.15.4
        │  │  │  ╭─ ⚙ run command git checkout -b bump/v0.15.4
        │  │  │  ╰─ ✔ run command git checkout -b bump/v0.15.4
        │  │  │  ╭─ ⚙ update cargo.toml
        │  │  │  │  ├─ ✔ updated Cargo.toml to version 0.15.4
        │  │  │  ╰─ ✔ update cargo.toml
        │  │  │  ╭─ ⚙ update cargo.lock
        │  │  │  │  ╭─ ⚙ run command cargo update --workspace
        │  │  │  │  ╰─ ✔ run command cargo update --workspace
        │  │  │  │  ├─ ✔ updated Cargo.lock
        │  │  │  ╰─ ✔ update cargo.lock
        │  │  │  ╭─ ⚙ commit and push
        │  │  │  │  ╭─ ⚙ run command git add Cargo.toml Cargo.lock
        │  │  │  │  ╰─ ✔ run command git add Cargo.toml Cargo.lock
        │  │  │  │  ╭─ ⚙ run command git commit -m bump version to 0.15.4
        │  │  │  │  ╰─ ✔ run command git commit -m bump version to 0.15.4
        │  │  │  │  ╭─ ⚙ run command git push -u origin HEAD
        │  │  │  │  │  ├─ ⚠ remote: You are not allowed to push code to this project.
        │  │  │  │  │  ├─ ⚠ fatal: unable to access 'https://git.example.com/example/project.git/': The requested URL returned error: 403
        │  │  │  │  │  ├─ ⨯ You are not allowed to push code to this project.
        │  │  │  │  ╰─ ⨯ run command git push -u origin HEAD
        │  │  │  ╰─ ⨯ commit and push
        │  │  ╰─ ⨯ bump
        │  ╰─ ⨯ bump
        │  ├─ ! run id: 019c75a8-e7a6-7bec-822a-e371c363d73e
        ╰─ ⨯ cli failed
        ");
    }

    #[test]
    fn test_console_macros() {
        let writer = MockWriter::new();
        let subscriber = Registry::default().with(ConsoleLayer::with_writer(writer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            let _span = tracing::info_span!("macros").entered();
            console!(notice, "A notice");
            console!(info, "Some info");
            console!(warn, "A warning");
            console!(error, "An error");
        });

        let output = strip_non_deterministic(&writer.to_string());
        insta::assert_snapshot!(output, @r###"
        ╭─ ⚙ macros
        │  ├─ ! A notice
        │  ├─ ✔ Some info
        │  ├─ ⚠ A warning
        │  ├─ ⨯ An error
        ╰─ ⨯ macros failed
        "###);
    }

    #[test]
    fn test_console_result() {
        let writer = MockWriter::new();
        let subscriber = Registry::default().with(ConsoleLayer::with_writer(writer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            let _span = tracing::info_span!("result").entered();
            let res: Result<&str, &str> = Ok("all good");
            assert_eq!(console!(res, "Operation 1").unwrap(), "all good");

            let res: Result<&str, &str> = Err("very bad");
            assert_eq!(console!(res, "Operation 2").unwrap_err(), "very bad");
        });

        let output = strip_non_deterministic(&writer.to_string());
        insta::assert_snapshot!(output, @r###"
        ╭─ ⚙ result
        │  ├─ ✔ Operation 1: all good
        │  ├─ ⨯ Operation 2: very bad
        ╰─ ⨯ result failed
        "###);
    }

    #[test]
    fn test_console_result_unit_type() {
        let writer = MockWriter::new();
        let subscriber = Registry::default().with(ConsoleLayer::with_writer(writer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            let _span = tracing::info_span!("result_unit").entered();
            let res: Result<(), &str> = Ok(());
            console!(res, "Operation without output").unwrap();
        });

        let output = strip_non_deterministic(&writer.to_string());
        insta::assert_snapshot!(output, @r###"
        ╭─ ⚙ result_unit
        │  ├─ ✔ Operation without output
        ╰─ ✔ result_unit succeeded
        "###);
    }

    #[test]
    fn test_console_result_format_args() {
        let writer = MockWriter::new();
        let subscriber = Registry::default().with(ConsoleLayer::with_writer(writer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            let _span = tracing::info_span!("result_args").entered();
            let res: Result<&str, &str> = Ok("all good");
            console!(res, "Operation {}", 1).unwrap();

            let res: Result<(), &str> = Err("very bad");
            console!(res, "Operation {} of {}", 2, 2).unwrap_err();
        });

        let output = strip_non_deterministic(&writer.to_string());
        insta::assert_snapshot!(output, @r###"
        ╭─ ⚙ result_args
        │  ├─ ✔ Operation 1: all good
        │  ├─ ⨯ Operation 2 of 2: very bad
        ╰─ ⨯ result_args failed
        "###);
    }

    #[test]
    fn test_complex_scenario() {
        let writer = MockWriter::new();
        let subscriber = Registry::default().with(ConsoleLayer::with_writer(writer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            let _cli = tracing::info_span!("cli", extra = "cli").entered();
            console!(notice, "run id: 928ac7fa-f9d3-4121-9401-b6edbc8369d4");
            let _gen = tracing::info_span!("gen").entered();
            {
                let _config = tracing::info_span!("configuration").entered();
                console!(info, "project root folder: /home/user/myapp");
                console!(
                    info,
                    "configuration loaded from /home/user/myapp/config.toml"
                );
            }
            {
                let _templates = tracing::info_span!("templates").entered();
                let _render = tracing::info_span!("render package", extra = "cli").entered();
                console!(notice, "file /home/user/myapp/build/cli.nix is up to date");
            }
            {
                let _deps = tracing::info_span!("dependencies").entered();
                console!(info, "1445 transitive dependencies found");
                tracing::error!("something");
            }
        });

        let output = strip_non_deterministic(&writer.to_string());
        insta::assert_snapshot!(output, @r"
        ╭─ ⚙ cli cli
        │  ├─ ! run id: 928ac7fa-f9d3-4121-9401-b6edbc8369d4
        │  ╭─ ⚙ gen
        │  │  ╭─ ⚙ configuration
        │  │  │  ├─ ✔ project root folder: /home/user/myapp
        │  │  │  ├─ ✔ configuration loaded from /home/user/myapp/config.toml
        │  │  ╰─ ✔ configuration
        │  │  ╭─ ⚙ templates
        │  │  │  ╭─ ⚙ render package cli
        │  │  │  │  ├─ ! file /home/user/myapp/build/cli.nix is up to date
        │  │  │  ╰─ ✔ render package cli
        │  │  ╰─ ✔ templates
        │  │  ╭─ ⚙ dependencies
        │  │  │  ├─ ✔ 1445 transitive dependencies found
        │  │  │  ├─ ⨯ 
        │  │  ╰─ ⨯ dependencies
        │  ╰─ ⨯ gen
        ╰─ ⨯ cli cli failed
        ");
    }

    #[test]
    fn test_custom_colors() {
        use owo_colors::colors;

        let writer = MockWriter::new();
        let colors = ColorScheme::default()
            .with_success_color(colors::BrightGreen)
            .with_error_color(colors::BrightRed)
            .with_tree_color(colors::Magenta);

        let layer = ConsoleLayer::with_writer(writer.clone()).with_colors(colors);
        let subscriber = Registry::default().with(layer);

        tracing::subscriber::with_default(subscriber, || {
            let _span = tracing::info_span!("custom_colors").entered();
            console!(info, "Testing custom colors");
            console!(error, "This is an error with custom colors");
        });

        let output = strip_non_deterministic(&writer.to_string());
        // We just verify the structure is correct - the colors themselves
        // are encoded as ANSI codes which are stripped by our test helper
        insta::assert_snapshot!(output, @r###"
        ╭─ ⚙ custom_colors
        │  ├─ ✔ Testing custom colors
        │  ├─ ⨯ This is an error with custom colors
        ╰─ ⨯ custom_colors failed
        "###);
    }

    #[test]
    fn test_color_scheme_builder() {
        // Test that the builder pattern works correctly
        let colors = ColorScheme::new()
            .with_success_color(owo_colors::colors::Green)
            .with_error_color(owo_colors::colors::Red)
            .with_warning_color(owo_colors::colors::Yellow)
            .with_notice_color(owo_colors::colors::Magenta);

        // Just verify we can create it without errors
        assert!(format!("{:?}", colors).contains("ColorScheme"));
    }

    #[test]
    fn test_colors_with_custom_status() {
        #[derive(Clone, Copy)]
        struct TestStatusFactory;

        impl StatusFactory<DefaultStatus> for TestStatusFactory {
            fn group(&self) -> DefaultStatus {
                DefaultStatus::Group
            }
            fn success(&self) -> DefaultStatus {
                DefaultStatus::Success
            }
            fn failure(&self) -> DefaultStatus {
                DefaultStatus::Failure
            }
        }

        let writer = MockWriter::new();
        let colors = ColorScheme::default().with_success_color(owo_colors::colors::BrightGreen);

        let layer = ConsoleLayer::with_writer(writer.clone())
            .with_colors(colors)
            .with_status(TestStatusFactory);

        let subscriber = Registry::default().with(layer);

        tracing::subscriber::with_default(subscriber, || {
            let _span = tracing::info_span!("test").entered();
            console!(info, "Works with custom status factory");
        });

        let output = strip_non_deterministic(&writer.to_string());
        insta::assert_snapshot!(output, @r###"
        ╭─ ⚙ test
        │  ├─ ✔ Works with custom status factory
        ╰─ ✔ test succeeded
        "###);
    }

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration(Duration::from_secs(1)), "1.0s");
        assert_eq!(format_duration(Duration::from_millis(1500)), "1.5s");
        assert_eq!(format_duration(Duration::from_secs(65)), "65.0s");
        assert_eq!(format_duration(Duration::from_secs_f64(1.06)), "1.1s");
    }

    #[test]
    fn test_format_duration_milliseconds() {
        assert_eq!(format_duration(Duration::from_millis(1)), "1.0ms");
        assert_eq!(format_duration(Duration::from_millis(500)), "500.0ms");
        assert_eq!(format_duration(Duration::from_millis(999)), "999.0ms");
        assert_eq!(format_duration(Duration::from_micros(1500)), "1.5ms");
    }

    #[test]
    fn test_format_duration_microseconds() {
        assert_eq!(format_duration(Duration::from_micros(1)), "1.0µs");
        assert_eq!(format_duration(Duration::from_micros(500)), "500.0µs");
        assert_eq!(format_duration(Duration::from_micros(999)), "999.0µs");
        assert_eq!(format_duration(Duration::from_nanos(1500)), "1.5µs");
    }

    #[test]
    fn test_format_duration_nanoseconds() {
        assert_eq!(format_duration(Duration::from_nanos(1)), "1ns");
        assert_eq!(format_duration(Duration::from_nanos(500)), "500ns");
        assert_eq!(format_duration(Duration::from_nanos(999)), "999ns");
    }

    #[test]
    fn test_format_duration_zero() {
        assert_eq!(format_duration(Duration::ZERO), "0ns");
    }

    #[test]
    fn test_format_duration_no_1000_units_at_boundaries() {
        // Values that would round to "1000.0ms" are promoted to seconds
        assert_eq!(format_duration(Duration::from_nanos(999_999_999)), "1.0s");
        assert_eq!(format_duration(Duration::from_nanos(999_950_000)), "1.0s");

        // Values that would round to "1000.0µs" are promoted to milliseconds
        assert_eq!(format_duration(Duration::from_nanos(999_999)), "1.0ms");
        assert_eq!(format_duration(Duration::from_nanos(999_950)), "1.0ms");

        // Just below the promotion threshold stays in the lower tier
        assert_eq!(
            format_duration(Duration::from_nanos(999_949_999)),
            "999.9ms"
        );
        assert_eq!(format_duration(Duration::from_nanos(999_949)), "999.9µs");

        // At exact unit boundaries
        assert_eq!(format_duration(Duration::from_nanos(1_000_000_000)), "1.0s");
        assert_eq!(format_duration(Duration::from_nanos(1_000_000)), "1.0ms");
        assert_eq!(format_duration(Duration::from_nanos(1_000)), "1.0µs");
    }
}
