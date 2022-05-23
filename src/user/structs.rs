#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::ffi_types::BOOL;
use crate::kernel::decl::{HINSTANCE, HIWORD, LCID, LOBYTE, LOWORD, MAKEDWORD};
use crate::user::decl::{
	DispfNup, HBITMAP, HBRUSH, HCURSOR, HDC, HICON, HMENU, HWND, HwndHmenu,
	HwndPlace, WNDPROC,
};
use crate::user::privs::{
	CCHDEVICENAME, CCHFORMNAME, CCHILDREN_TITLEBAR, DM_SPECVERSION,
};

/// [`ACCEL`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-accel)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub struct ACCEL {
	pub fVirt: co::ACCELF,
	pub key: co::VK,
	pub cmd: u16,
}

/// [`ALTTABINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-alttabinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct ALTTABINFO {
	cbSize: u32,
	pub cItems: i32,
	pub cColumns: i32,
	pub cRows: i32,
	pub iColFocus: i32,
	pub iRowFocus: i32,
	pub cxItem: i32,
	pub cyItem: i32,
	pub ptStart: POINT,
}

impl_default_with_size!(ALTTABINFO, cbSize);

/// [`ATOM`](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#atom)
/// returned by [`RegisterClassEx`](crate::RegisterClassEx).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ATOM(pub(crate) u16);

impl std::fmt::Display for ATOM {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

/// [`COLORREF`](https://docs.microsoft.com/en-us/windows/win32/gdi/colorref)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct COLORREF(pub(crate) u32);

impl From<co::CLR> for COLORREF {
	fn from(v: co::CLR) -> Self {
		Self(v.0)
	}
}

impl std::fmt::Display for COLORREF {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "R {}, G {}, B {}",
			self.GetRValue(), self.GetGValue(), self.GetBValue())
	}
}

impl COLORREF {
	/// Creates a new `COLORREF` object with the given color intensities.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::COLORREF;
	///
	/// let color = COLORREF::new(0xff, 0x80, 0x00);
	/// ```
	#[must_use]
	pub const fn new(red: u8, green: u8, blue: u8) -> COLORREF {
		Self(red as u32 | ((green as u32) << 8) | ((blue as u32) << 16))
	}

	/// Creates an array of `COLORREF` objects with the given color intensities.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::COLORREF;
	///
	/// let colors: [COLORREF; 2] = COLORREF::new_array(&[
	///     (0xff, 0xb2, 0x80),
	///     (0x00, 0xa0, 0x40),
	/// ]);
	/// ```
	#[must_use]
	pub fn new_array<const N: usize>(rgbs: &[(u8, u8, u8); N]) -> [COLORREF; N] {
		let mut arr = [Self::new(0, 0, 0); N];
		for (i, rgb) in rgbs.iter().enumerate() {
			arr[i] = Self::new(rgb.0, rgb.1, rgb.2);
		}
		arr
	}

	/// Retrieves the red intensity. Originally
	/// [`GetRValue`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getrvalue)
	/// macro.
	#[must_use]
	pub const fn GetRValue(self) -> u8 {
		LOBYTE(LOWORD(self.0))
	}

	/// Retrieves the green intensity. Originally
	/// [`GetGValue`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getgvalue)
	/// macro.
	#[must_use]
	pub const fn GetGValue(self) -> u8 {
		LOBYTE(LOWORD(self.0 >> 8))
	}

	/// Retrieves the blue intensity. Originally
	/// [`GetBValue`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbvalue)
	/// macro.
	#[must_use]
	pub const fn GetBValue(self) -> u8 {
		LOBYTE(LOWORD(self.0 >> 16))
	}
}

/// [`COMBOBOXINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-comboboxinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct COMBOBOXINFO {
	cbSize: u32,
	pub rcItem: RECT,
	pub rcButton: RECT,
	pub stateButton: co::STATE_SYSTEM,
	pub hwndCombo: HWND,
	pub hwndItem: HWND,
	pub hwndList: HWND,
}

impl_default_with_size!(COMBOBOXINFO, cbSize);

/// [`COMPAREITEMSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-compareitemstruct)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct COMPAREITEMSTRUCT {
	pub CtlType: co::ODT_C,
	pub CtlID: u32,
	pub hwndItem: HWND,
	pub itemID1: u32,
	pub itemData1: usize,
	pub itemID2: u32,
	pub itemData2: usize,
	pub dwLocaleId: LCID,
}

impl_default!(COMPAREITEMSTRUCT);

/// [`CREATESTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct CREATESTRUCT<'a, 'b> {
	pub lpCreateParams: isize,
	pub hInstance: HINSTANCE,
	pub hMenu: HMENU,
	pub hwndParent: HWND,
	pub cy: i32,
	pub cx: i32,
	pub y: i32,
	pub x: i32,
	pub style: co::WS,
	lpszName: *mut u16,
	lpszClass: *mut u16,
	pub dwExStyle: co::WS_EX,

	_lpszName: PhantomData<&'a mut u16>,
	_lpszClass: PhantomData<&'b mut u16>,
}

impl_default!(CREATESTRUCT, 'a, 'b);

impl<'a, 'b> CREATESTRUCT<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, lpszName, set_lpszName);
	pub_fn_string_ptr_get_set!('b, lpszClass, set_lpszClass);
}

/// [`DELETEITEMSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-deleteitemstruct)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct DELETEITEMSTRUCT {
	pub CtlType: co::ODT_C,
	pub CtlID: u32,
	pub itemID: u32,
	pub hwndItem: HWND,
	pub itemData: usize,
}

impl_default!(DELETEITEMSTRUCT);

/// [`DEVMODE`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-devmodew)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct DEVMODE {
	dmDeviceName: [u16; CCHDEVICENAME],
	dmSpecVersion: u16,
	pub dmDriverVersion: u16,
	dmSize: u16,
	pub dmDriverExtra: u16,
	pub dmFields: co::DM,
	union0: DEVMODE_union0,
	pub dmColor: co::DMCOLOR,
	pub dmDuplex: co::DMDUP,
	pub dmYResolution: u16,
	pub dmTTOption: co::DMTT,
	dmCollate: i16,
	dmFormName: [u16; CCHFORMNAME],
	pub dmLogPixels: u16,
	pub dmBitsPerPel: u32,
	pub dmPelsWidth: u32,
	pub dmPelsHeight: u32,
	union1: DEVMODE_union1,
	pub dmDisplayFrequency: u32,
	pub dmICMMethod: co::DMICMMETHOD,
	pub dmICMIntent: co::DMICM,
	pub dmMediaType: co::DMMEDIA,
	pub dmDitherType: co::DMDITHER,
	dmReserved1: u32,
	dmReserved2: u32,
	dmPanningWidth: u32,
	dmPanningHeight: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DEVMODE_printer {
	dmOrientation: co::DMORIENT,
	dmPaperSize: co::DMPAPER,
	dmPaperLength: i16,
	dmPaperWidth: i16,
	dmScale: i16,
	dmCopies: i16,
	dmDefaultSource: co::DMBIN,
	dmPrintQuality: co::DMRES,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DEVMODE_display {
	dmPosition: POINT,
	dmDisplayOrientation: co::DMDO,
	dmDisplayFixedOutput: co::DMDFO,
}

#[repr(C)]
union DEVMODE_union0 {
	printer: DEVMODE_printer,
	display: DEVMODE_display,
}

#[repr(C)]
union DEVMODE_union1 {
	dmDisplayFlags: co::DMDISPLAYFLAGS,
	dnNup: co::DMNUP,
}

impl Default for DEVMODE {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.dmSpecVersion = DM_SPECVERSION;
		obj.dmDriverVersion = DM_SPECVERSION;
		obj.dmSize = std::mem::size_of::<Self>() as _;
		obj
	}
}

impl DEVMODE {
	pub_fn_string_arr_get_set!(dmDeviceName, set_dmDeviceName);
	pub_fn_bool_get_set!(dmCollate, set_dmCollate);
	pub_fn_string_arr_get_set!(dmFormName, set_dmFormName);

	/// Sets the `dmDisplayFlags` or the `dmNup` field.
	pub fn set_dmDisplayFlags_dmNup(&mut self, val: DispfNup) {
		match val {
			DispfNup::Dispf(val) => self.union1.dmDisplayFlags = val,
			DispfNup::Nup(val) => self.union1.dnNup = val,
		}
	}

	/// Returns the `dmOrientation` printer field, which is part of an union.
	#[must_use]
	pub fn dmOrientation(&self) -> co::DMORIENT {
		unsafe { self.union0.printer.dmOrientation }
	}

	/// Sets the `dmOrientation` printer field, which is part of an union.
	pub fn set_dmOrientation(&mut self, val: co::DMORIENT) {
		self.union0.printer.dmOrientation = val;
	}

	/// Returns the `dmPaperSize` printer field, which is part of an union.
	#[must_use]
	pub fn dmPaperSize(&self) -> co::DMPAPER {
		unsafe { self.union0.printer.dmPaperSize }
	}

	/// Sets the `dmPaperSize` printer field, which is part of an union.
	pub fn set_dmPaperSize(&mut self, val: co::DMPAPER) {
		self.union0.printer.dmPaperSize = val;
	}

	/// Returns the `dmPaperLength` printer field, which is part of an union.
	#[must_use]
	pub fn dmPaperLength(&self) -> i16 {
		unsafe { self.union0.printer.dmPaperLength }
	}

	/// Sets the `dmPaperLength` printer field, which is part of an union.
	pub fn set_dmPaperLength(&mut self, val: i16) {
		self.union0.printer.dmPaperLength = val;
	}

	/// Returns the `dmPaperWidth` printer field, which is part of an union.
	#[must_use]
	pub fn dmPaperWidth(&self) -> i16 {
		unsafe { self.union0.printer.dmPaperWidth }
	}

	/// Sets the `dmPaperWidth` printer field, which is part of an union.
	pub fn set_dmPaperWidth(&mut self, val: i16) {
		self.union0.printer.dmPaperWidth = val;
	}

	/// Returns the `dmScale` printer field, which is part of an union.
	#[must_use]
	pub fn dmScale(&self) -> i16 {
		unsafe { self.union0.printer.dmScale }
	}

	/// Sets the `dmScale` printer field, which is part of an union.
	pub fn set_dmScale(&mut self, val: i16) {
		self.union0.printer.dmScale = val;
	}

	/// Returns the `dmCopies` printer field, which is part of an union.
	#[must_use]
	pub fn dmCopies(&self) -> i16 {
		unsafe { self.union0.printer.dmCopies }
	}

	/// Sets the `dmCopies` printer field, which is part of an union.
	pub fn set_dmCopies(&mut self, val: i16) {
		self.union0.printer.dmCopies = val;
	}

	/// Returns the `dmDefaultSource` printer field, which is part of an union.
	#[must_use]
	pub fn dmDefaultSource(&self) -> co::DMBIN {
		unsafe { self.union0.printer.dmDefaultSource }
	}

	/// Sets the `dmDefaultSource` printer field, which is part of an union.
	pub fn set_dmDefaultSource(&mut self, val: co::DMBIN) {
		self.union0.printer.dmDefaultSource = val;
	}

	/// Returns the `dmPrintQuality` printer field, which is part of an union.
	#[must_use]
	pub fn dmPrintQuality(&self) -> co::DMRES {
		unsafe { self.union0.printer.dmPrintQuality }
	}

	/// Sets the `dmPrintQuality` printer field, which is part of an union.
	pub fn set_dmPrintQuality(&mut self, val: co::DMRES) {
		self.union0.printer.dmPrintQuality = val;
	}

	/// Returns the `dmPosition` display field, which is part of an union.
	#[must_use]
	pub fn dmPosition(&self) -> POINT {
		unsafe { self.union0.display.dmPosition }
	}

	/// Sets the `dmPosition` display field, which is part of an union.
	pub fn set_dmPosition(&mut self, val: POINT) {
		self.union0.display.dmPosition = val;
	}

	/// Returns the `dmDisplayOrientation` display field, which is part of an
	/// union.
	#[must_use]
	pub fn dmDisplayOrientation(&self) -> co::DMDO {
		unsafe { self.union0.display.dmDisplayOrientation }
	}

	/// Sets the `dmDisplayOrientation` display field, which is part of an union.
	pub fn set_dmDisplayOrientation(&mut self, val: co::DMDO) {
		self.union0.display.dmDisplayOrientation = val;
	}

	/// Returns the `dmDisplayFixedOutput` display field, which is part of an
	/// union.
	#[must_use]
	pub fn dmDisplayFixedOutput(&self) -> co::DMDFO {
		unsafe { self.union0.display.dmDisplayFixedOutput }
	}

	/// Sets the `dmDisplayFixedOutput` display field, which is part of an union.
	pub fn set_dmDisplayFixedOutput(&mut self, val: co::DMDFO) {
		self.union0.display.dmDisplayFixedOutput = val;
	}
}

/// [`DISPLAY_DEVICE`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-display_devicew)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct DISPLAY_DEVICE {
	cb: u32,
	DeviceName: [u16; 32],
	DeviceString: [u16; 128],
	pub StateFlags: co::DISPLAY_DEVICE,
	DeviceID: [u16; 128],
	DeviceKey: [u16; 128],
}

impl_default_with_size!(DISPLAY_DEVICE, cb);

impl DISPLAY_DEVICE {
	pub_fn_string_arr_get_set!(DeviceName, set_DeviceName);
	pub_fn_string_arr_get_set!(DeviceString, set_DeviceString);
	pub_fn_string_arr_get_set!(DeviceID, set_DeviceID);
	pub_fn_string_arr_get_set!(DeviceKey, set_DeviceKey);
}

/// [`DRAWITEMSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-drawitemstruct)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct DRAWITEMSTRUCT {
	pub CtlType: co::ODT,
	pub CtlID: u32,
	pub itemID: u32,
	pub itemAction: co::ODA,
	pub itemState: co::ODS,
	pub hwndItem: HWND,
	pub hDC: HDC,
	pub rcItem: RECT,
	pub itemData: usize,
}

impl_default!(DRAWITEMSTRUCT);

/// [`MSG`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-msg)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
#[derive(Clone)]
pub struct MSG {
	pub hwnd: HWND,
	pub message: co::WM,
	pub wParam: usize,
	pub lParam: isize,
	pub time: u32,
	pub pt: POINT,
}

impl_default!(MSG);

/// [`GUITHREADINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-guithreadinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct GUITHREADINFO {
	cbSize: u32,
	pub flags: co::GUI,
	pub hwndActive: HWND,
	pub hwndFocus: HWND,
	pub hwndCapture: HWND,
	pub hwndMenuOwner: HWND,
	pub hwndMoveSize: HWND,
	pub hwndCaret: HWND,
	pub rcCaret: RECT,
}

impl_default_with_size!(GUITHREADINFO, cbSize);

/// [`HELPINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-helpinfo)
/// struct.
///
/// You cannot directly instantiate this object.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct HELPINFO {
	cbSize: u32,
	pub iContextType: co::HELPINFO,
	pub iCtrlId: i32,
	hItemHandle: usize, // HWND|HMENU
	pub dwContextId: u32,
	pub MousePos: POINT,
}

impl HELPINFO {
	#[must_use]
	pub fn hItemHandle(&self) -> HwndHmenu {
		match self.iContextType {
			co::HELPINFO::WINDOW => HwndHmenu::Hwnd(HWND(self.hItemHandle as _)),
			_ => HwndHmenu::Hmenu(HMENU(self.hItemHandle as _)),
		}
	}
}

/// [`MENUBARINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-menubarinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct MENUBARINFO {
	cbSize: u32,
	pub rcBar: RECT,
	pub hMenu: HMENU,
	pub hwndMenu: HWND,
	fBarFocused: BOOL,
	fFocused: BOOL,
}

impl_default_with_size!(MENUBARINFO, cbSize);

impl MENUBARINFO {
	pub_fn_bool_get_set!(fBarFocused, set_fBarFocused);
	pub_fn_bool_get_set!(fFocused, set_fFocused);
}

/// [`MENUINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-menuinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct MENUINFO {
	cbSize: u32,
	pub fMask: co::MIM,
	pub dwStyle: co::MNS,
	pub cyMax: u32,
	pub hbrBack: HBRUSH,
	pub dwContextHelpID: u32,
	pub dwMenuData: usize,
}

impl_default_with_size!(MENUINFO, cbSize);

/// [`MENUITEMINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-menuiteminfow)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct MENUITEMINFO {
	cbSize: u32,
	pub fMask: co::MIIM,
	pub fType: co::MFT,
	pub fState: co::MFS,
	pub wID: u32,
	pub hSubMenu: HMENU,
	pub hbmpChecked: HBITMAP,
	pub hbmpUnchecked: HBITMAP,
	pub dwItemData: usize,
	pub dwTypeData: *mut u16,
	pub cch: u32,
	pub hbmpItem: HBITMAP,
}

impl_default_with_size!(MENUITEMINFO, cbSize);

/// [`MINMAXINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-minmaxinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct MINMAXINFO {
	ptReserved: POINT,
	/// The maximized width (x member) and the maximized height (y member) of
	/// the window. For top-level windows, this value is based on the width of
	/// the primary monitor.
	pub ptMaxSize: POINT,
	/// The position of the left side of the maximized window (x member) and the
	/// position of the top of the maximized window (y member). For top-level
	/// windows, this value is based on the position of the primary monitor.
	pub ptMaxPosition: POINT,
	/// The minimum tracking width (x member) and the minimum tracking height (y
	/// member) of the window. This value can be obtained programmatically from
	/// the system metrics [`SM::CXMINTRACK`](crate::co::SM::CXMINTRACK) and
	/// [`SM::CYMINTRACK`](crate::co::SM::CYMINTRACK) (see the
	/// [`GetSystemMetrics`](crate::GetSystemMetrics) function).
	pub ptMinTrackSize: POINT,
	/// The maximum tracking width (x member) and the maximum tracking height (y
	/// member) of the window. This value is based on the size of the virtual
	/// screen and can be obtained programmatically from the system metrics
	/// [`SM::CXMAXTRACK`](crate::co::SM::CXMAXTRACK) and
	/// [`SM::CYMAXTRACK`](crate::co::SM::CYMAXTRACK) (see the
	/// [`GetSystemMetrics`](crate::GetSystemMetrics) function).
	pub ptMaxTrackSize: POINT,
}

/// [`MONITORINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-monitorinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
#[derive(Clone)]
pub struct MONITORINFO {
	cbSize: u32,
	pub rcMonitor: RECT,
	pub rcWork: RECT,
	pub dwFlags: co::MONITORINFOF,
}

impl_default_with_size!(MONITORINFO, cbSize);

/// [`NCCALCSIZE_PARAMS`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nccalcsize_params)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct NCCALCSIZE_PARAMS<'a> {
	pub rgrc: [RECT; 3],
	lppos: *mut WINDOWPOS,

	_lppos: PhantomData<&'a mut WINDOWPOS>,
}

impl<'a> NCCALCSIZE_PARAMS<'a> {
	pub_fn_ptr_get_set!('a, lppos, set_lppos, WINDOWPOS);
}

impl_default!(NCCALCSIZE_PARAMS, 'a);

/// [`PAINTSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct PAINTSTRUCT {
	pub hdc: HDC,
	pub fErase: u32,
	pub rcPaint: RECT,
	fRestore: u32,
	fIncUpdate: u32,
	rgbReserved: [u8; 32],
}

impl_default!(PAINTSTRUCT);

/// [`POINT`](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-point)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct POINT {
	pub x: i32,
	pub y: i32,
}

impl std::fmt::Display for POINT {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "x {}, y {}", self.x, self.y)
	}
}

impl POINT {
	/// Creates a new `POINT`.
	#[must_use]
	pub const fn new(x: i32, y: i32) -> POINT {
		Self { x, y }
	}

	/// Creates a new `POINT` by splitting `val` into two `u16` values, with
	/// [`LOWORD`](crate::LOWORD) and [`HIWORD`](crate::HIWORD).
	#[must_use]
	pub const fn from_u32(val: u32) -> POINT {
		Self::new(LOWORD(val) as _, HIWORD(val) as _)
	}

	/// Tells whether the struct contains exactly the given values.
	#[must_use]
	pub const fn is(&self, x: i32, y: i32) -> bool {
		self.x == x && self.y == y
	}

	/// Calls [`MAKEDWORD`](crate::MAKEDWORD) with `x` and `y`.
	#[must_use]
	pub const fn into_u32(&self) -> u32 {
		MAKEDWORD(self.x as _, self.y as _)
	}
}

/// [`RECT`](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct RECT {
	pub left: i32,
	pub top: i32,
	pub right: i32,
	pub bottom: i32,
}

impl std::fmt::Display for RECT {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "left {}, top {}, right {}, bottom {}",
			self.left, self.top, self.right, self.bottom)
	}
}

impl RECT {
	/// Tells whether the struct contains exactly the given values.
	#[must_use]
	pub const fn is(&self, left: i32, top: i32, right: i32, bottom: i32) -> bool {
		self.left == left && self.top == top && self.right == right && self.bottom == bottom
	}
}

/// [`SCROLLINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-scrollinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
#[derive(Clone)]
pub struct SCROLLINFO {
	cbSize: u32,
	pub fMask: co::SIF,
	pub nMin: i32,
	pub nMax: i32,
	pub nPage: u32,
	pub nPos: i32,
	pub nTrackPos: i32,
}

impl_default_with_size!(SCROLLINFO, cbSize);

/// [`SIZE`](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-size)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct SIZE {
	pub cx: i32,
	pub cy: i32,
}

impl std::fmt::Display for SIZE {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "cx {}, cy {}", self.cx, self.cy)
	}
}

impl SIZE {
	/// Creates a new `SIZE`.
	#[must_use]
	pub const fn new(cx: i32, cy: i32) -> SIZE {
		Self { cx, cy }
	}

	/// Creates a new `SIZE` by splitting `val` into two `u16` values, with
	/// [`LOWORD`](crate::LOWORD) and [`HIWORD`](crate::HIWORD).
	#[must_use]
	pub const fn from_u32(val: u32) -> SIZE {
		Self::new(LOWORD(val) as _, HIWORD(val) as _)
	}

	/// Tells whether the struct contains exactly the given values.
	#[must_use]
	pub const fn is(&self, cx: i32, cy: i32) -> bool {
		self.cx == cx && self.cy == cy
	}

	/// Calls [`MAKEDWORD`](crate::MAKEDWORD) with `cx` and `cy`.
	#[must_use]
	pub const fn into_u32(&self) -> u32 {
		MAKEDWORD(self.cx as _, self.cy as _)
	}
}

/// [`STYLESTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-stylestruct)
/// struct.
///
/// You cannot directly instantiate this object.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
#[derive(Default)]
pub struct STYLESTRUCT {
	styleOld: u32, // both fields contain WS and WS_EX mixed
	styleNew: u32,
}

impl STYLESTRUCT {
	/// Returns the [`WS`](crate::co::WS) of `styleOld` field.
	#[must_use]
	pub fn styleOld_WS(&self) -> co::WS {
		co::WS(self.styleOld)
	}

	/// Returns the [`WS_EX`](crate::co::WS_EX) of `styleOld` field.
	#[must_use]
	pub fn styleOld_WSEX(&self) -> co::WS_EX {
		co::WS_EX(self.styleOld)
	}

	/// Returns the [`WS`](crate::co::WS) of `styleNew` field.
	#[must_use]
	pub fn styleNew_WS(&self) -> co::WS {
		co::WS(self.styleNew)
	}

	/// Returns the [`WS_EX`](crate::co::WS_EX) of `styleNew` field.
	#[must_use]
	pub fn styleNew_WSEX(&self) -> co::WS_EX {
		co::WS_EX(self.styleNew)
	}
}

/// [`TITLEBARINFOEX`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-titlebarinfoex)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct TITLEBARINFOEX {
	cbSize: u32,
	pub rcTitleBar: RECT,
	pub rgstate: [co::STATE_SYSTEM; CCHILDREN_TITLEBAR + 1],
	pub rgrect: [RECT; CCHILDREN_TITLEBAR + 1],
}

impl_default_with_size!(TITLEBARINFOEX, cbSize);

/// [`TRACKMOUSEEVENT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-trackmouseevent)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct TRACKMOUSEEVENT {
	cbSize: u32,
	pub dwFlags: co::TME,
	pub hwndTrack: HWND,
	pub dwHoverTime: u32,
}

impl_default_with_size!(TRACKMOUSEEVENT, cbSize);

/// [`WINDOWINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct WINDOWINFO {
	cbSize: u32,
	pub rcWindow: RECT,
	pub rcClient: RECT,
	pub dwStyle: co::WS,
	pub dwExStyle: co::WS_EX,
	pub dwWindowStatus: u32,
	pub cxWindowBorders: u32,
	pub cyWindowBorders: u32,
	pub atomWindowType: ATOM,
	pub wCreatorVersion: u16,
}

impl_default_with_size!(WINDOWINFO, cbSize);

/// [`WINDOWPLACEMENT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowplacement)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct WINDOWPLACEMENT {
	length: u32,
	pub flags: co::WPF,
	pub showCmd: co::SW,
	pub ptMinPosition: POINT,
	pub ptMaxPosition: POINT,
	pub rcNormalPosition: RECT,
	pub rcDevice: RECT,
}

impl_default_with_size!(WINDOWPLACEMENT, length);

/// [`WINDOWPOS`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowpos)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct WINDOWPOS {
	pub hwnd: HWND,
	hwndInsertAfter: isize,
	pub x: i32,
	pub y: i32,
	pub cx: i32,
	pub cy: i32,
	pub flags: co::SWP,
}

impl_default!(WINDOWPOS);

impl WINDOWPOS {
	/// Returns the `hwndInsertAfter` field.
	#[must_use]
	pub fn hwndInsertAfter(&self) -> HwndPlace {
		match self.hwndInsertAfter {
			0 | 1 | -1 | -2 => HwndPlace::Place(co::HWND_PLACE(self.hwndInsertAfter)),
			_ => HwndPlace::Hwnd(HWND(self.hwndInsertAfter as _)),
		}
	}

	/// Sets the `hwndInsertAfter` field.
	pub fn set_hwndInsertAfter(&mut self, hwnd: HwndPlace) {
		self.hwndInsertAfter = match hwnd {
			HwndPlace::Hwnd(h) => h.0 as _,
			HwndPlace::Place(v) => v.into(),
			HwndPlace::None => 0,
		};
	}
}

/// [`WNDCLASSEX`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[repr(C)]
pub struct WNDCLASSEX<'a> {
	cbSize: u32,
	pub style: co::CS,
	pub lpfnWndProc: Option<WNDPROC>,
	pub cbClsExtra: i32,
	pub cbWndExtra: i32,
	pub hInstance: HINSTANCE,
	pub hIcon: HICON,
	pub hCursor: HCURSOR,
	pub hbrBackground: HBRUSH,
	lpszMenuName: *mut u16, // u16 resource ID
	lpszClassName: *mut u16,
	pub hIconSm: HICON,

	_lpszClassName: PhantomData<&'a mut u16>,
}

impl_default_with_size!(WNDCLASSEX, cbSize, 'a);

impl<'a> WNDCLASSEX<'a> {
	pub_fn_resource_id_get_set!(lpszMenuName, set_lpszMenuName);
	pub_fn_string_ptr_get_set!('a, lpszClassName, set_lpszClassName);
}
