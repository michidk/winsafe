use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::dlg_base::DlgBase;
use crate::gui::events::WindowEventsAll;
use crate::gui::runtime_error::RunResult;
use crate::kernel::decl::{ErrResult, HINSTANCE, IdStr};
use crate::msg::wm;
use crate::prelude::{GuiEvents, KernelHinstance, UserHinstance, UserHwnd};
use crate::user::decl::{HWND, PostQuitMessage};

struct Obj { // actual fields of DlgMain
	dlg_base: DlgBase,
	icon_id: Option<u16>,
	accel_table_id: Option<u16>,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// A dialog-based main window.
#[derive(Clone)]
pub(in crate::gui) struct DlgMain(Pin<Arc<Obj>>);

impl DlgMain {
	pub(in crate::gui) fn new(
		dialog_id: u16,
		icon_id: Option<u16>,
		accel_table_id: Option<u16>) -> Self
	{
		let new_self = Self(
			Arc::pin(
				Obj {
					dlg_base: DlgBase::new(None, dialog_id),
					icon_id,
					accel_table_id,
					_pin: PhantomPinned,
				},
			),
		);
		new_self.default_message_handlers();
		new_self
	}

	pub(in crate::gui) unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		self.0.dlg_base.as_base()
	}

	pub(in crate::gui) fn hwnd(&self) -> HWND {
		self.0.dlg_base.hwnd()
	}

	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		self.0.dlg_base.on()
	}

	pub(in crate::gui) fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static,
	{
		self.0.dlg_base.spawn_new_thread(func);
	}

	pub(in crate::gui) fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static
	{
		self.0.dlg_base.run_ui_thread(func);
	}

	pub(in crate::gui) fn run_main(&self,
		cmd_show: Option<co::SW>) -> RunResult<i32>
	{
		self.0.dlg_base.create_dialog_param();
		let hinst = HINSTANCE::GetModuleHandle(None).unwrap();
		let haccel = self.0.accel_table_id
			.map(|id| hinst.LoadAccelerators(IdStr::Id(id))) // resources are automatically freed
			.transpose()
			.unwrap();

		self.set_icon_if_any(hinst);
		self.hwnd().ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));

		Base::run_main_loop(haccel) // blocks until window is closed
	}

	fn default_message_handlers(&self) {
		let self2 = self.clone();
		self.on().wm_close(move || {
			self2.hwnd().DestroyWindow().unwrap();
			Ok(())
		});

		self.on().wm_nc_destroy(|| {
			PostQuitMessage(0);
			Ok(())
		});
	}

	fn set_icon_if_any(&self, hinst: HINSTANCE) {
		// If an icon ID was specified, load it from the resources.
		// Resource icons are automatically released by the system.
		if let Some(id) = self.0.icon_id {
			self.hwnd().SendMessage(
				wm::SetIcon {
					hicon: hinst.LoadImageIcon(id, 16, 16, co::LR::DEFAULTCOLOR).unwrap(),
					size: co::ICON_SZ::SMALL,
				},
			);

			self.hwnd().SendMessage(
				wm::SetIcon {
					hicon: hinst.LoadImageIcon(id, 32, 32, co::LR::DEFAULTCOLOR).unwrap(),
					size: co::ICON_SZ::BIG,
				},
			);
		}
	}
}
