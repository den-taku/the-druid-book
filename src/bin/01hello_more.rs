use druid::widget::{Align, Flex, Label, Padding};
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(())?;
    Ok(())
}

fn build_ui() -> impl Widget<()> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top left"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom left")), 1.0),
                1.0,
            )
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top right"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom right")), 1.0),
                1.0,
            ),
    )
}
