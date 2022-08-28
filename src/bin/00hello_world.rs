use druid::widget::Label;
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(())?;
    Ok(())
}

fn build_ui() -> impl Widget<()> {
    Label::new("Hello, world")
}
