//! Events exposed by windows and controls, which allow the handling of native
//! Windows messages.

mod base_events_proxy;
mod button_events;
mod combo_box_events;
mod date_time_picker_events;
mod edit_events;
mod func_store;
mod label_events;
mod list_box_events;
mod list_view_events;
mod month_calendar_events;
mod radio_group_events;
mod status_bar_events;
mod trackbar_events;
mod tree_view_events;
mod window_events_all;
mod window_events;

pub use button_events::ButtonEvents;
pub use combo_box_events::ComboBoxEvents;
pub use date_time_picker_events::DateTimePickerEvents;
pub use edit_events::EditEvents;
pub use label_events::LabelEvents;
pub use list_box_events::ListBoxEvents;
pub use list_view_events::ListViewEvents;
pub use month_calendar_events::MonthCalendarEvents;
pub use radio_group_events::RadioGroupEvents;
pub use status_bar_events::StatusBarEvents;
pub use trackbar_events::TrackbarEvents;
pub use tree_view_events::TreeViewEvents;
pub use window_events_all::WindowEventsAll;
pub use window_events::WindowEvents;
pub(in crate::gui) use window_events::ProcessResult;

pub(in crate::gui) mod traits {
	pub use super::window_events::GuiEvents;
}
