//! `Widget`
//! - `event`: message (only on event coming, we can change our model)
//! - `update`: if `event` modify the data, `update` is called to update internal data and call `layout` if needed
//! - `layout`: determines where to position each widget
//! - `paint`: after `layout` calling, draw widgets
//! - `lifecycle`: ? (Additional function)
//!
//! `WidgetExt`
//! - Additional builder-style methods for supporting widgets customization

fn main() {}
