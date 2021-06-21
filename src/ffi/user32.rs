//! Raw bindings to user32.lib functions.

use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PFUNC, PSTR, PVOID};

#[link(name = "user32")]
extern "system" {
	pub fn AdjustWindowRectEx(_: PVOID, _: u32, _: BOOL, _: u32) -> BOOL;
	pub fn AppendMenuW(_: HANDLE, _: u32, _: usize, _: PCSTR) -> BOOL;
	pub fn ArrangeIconicWindows(_: HANDLE) -> u32;
	pub fn BeginDeferWindowPos(_: i32) -> HANDLE;
	pub fn BeginPaint(_: HANDLE, _: PVOID) -> HANDLE;
	pub fn BringWindowToTop(_: HANDLE) -> BOOL;
	pub fn CallNextHookEx(_: HANDLE, _: i32, _: usize, _: isize) -> isize;
	pub fn CheckMenuItem(_: HANDLE, _: u32, _: u32) -> i32;
	pub fn ChildWindowFromPoint(_: HANDLE, _: i32, _: i32) -> HANDLE;
	pub fn ClientToScreen(_: HANDLE, _: PVOID) -> BOOL;
	pub fn ClipCursor(_: PCVOID) -> BOOL;
	pub fn CloseClipboard() -> BOOL;
	pub fn CloseWindow(_: HANDLE) -> BOOL;
	pub fn CopyIcon(_: HANDLE) -> HANDLE;
	pub fn CreateAcceleratorTableW(_: PVOID, _: i32) -> HANDLE;
	pub fn CreateDialogParamW(_: HANDLE, _: PCSTR, _: HANDLE, _: PFUNC, _: isize) -> HANDLE;
	pub fn CreateMenu() -> HANDLE;
	pub fn CreatePopupMenu() -> HANDLE;
	pub fn CreateWindowExW(_: u32, _: PCSTR, _: PCSTR, _: u32, _: i32, _: i32, _: i32, _: i32, _: HANDLE, _: HANDLE, _: HANDLE, _: PVOID) -> HANDLE;
	pub fn DeferWindowPos(_: HANDLE, _: HANDLE, _: HANDLE, _: i32, _: i32, _: i32, _: i32, _: u32) -> HANDLE;
	pub fn DefWindowProcW(_: HANDLE, _: u32, _: usize, _: isize) -> isize;
	pub fn DeleteMenu(_: HANDLE, _: u32, _: u32) -> BOOL;
	pub fn DestroyAcceleratorTable(_: HANDLE) -> BOOL;
	pub fn DestroyCursor(_: HANDLE) -> BOOL;
	pub fn DestroyIcon(_: HANDLE) -> BOOL;
	pub fn DestroyMenu(_: HANDLE) -> BOOL;
	pub fn DestroyWindow(_: HANDLE) -> BOOL;
	pub fn DialogBoxParamW(_: HANDLE, _: PCSTR, _: HANDLE, _: PFUNC, _: isize) -> isize;
	pub fn DispatchMessageW(_: PCVOID) -> isize;
	pub fn EmptyClipboard() -> BOOL;
	pub fn EnableMenuItem(_: HANDLE, _: u32, _: u32) -> BOOL;
	pub fn EnableWindow(_: HANDLE, _: BOOL) -> BOOL;
	pub fn EndDeferWindowPos(_: HANDLE) -> BOOL;
	pub fn EndDialog(_: HANDLE, _: isize) -> BOOL;
	pub fn EndPaint(_: HANDLE, _: PCVOID) -> BOOL;
	pub fn EnumChildWindows(_: HANDLE, _: PFUNC, _: isize) -> BOOL;
	pub fn FindWindowExW(_: HANDLE, _: HANDLE, _: PCSTR, _: PCSTR) -> HANDLE;
	pub fn FindWindowW(_: PCSTR, _: PCSTR) -> HANDLE;
	pub fn GetActiveWindow() -> HANDLE;
	pub fn GetAltTabInfoW(_: HANDLE, _: i32, _: PVOID, _: PSTR, _: u32) -> BOOL;
	pub fn GetAncestor(_: HANDLE, _: u32) -> HANDLE;
	pub fn GetAsyncKeyState(_: i32) -> i16;
	pub fn GetCapture() -> HANDLE;
	pub fn GetClassInfoExW(_: HANDLE, _: PCSTR, _: PVOID) -> BOOL;
	pub fn GetClassLongPtrW(_: HANDLE, _: i32) -> usize;
	pub fn GetClientRect(_: HANDLE, _: PVOID) -> BOOL;
	pub fn GetClipCursor(_: PVOID) -> BOOL;
	pub fn GetCursorPos(_: PVOID) -> BOOL;
	pub fn GetDC(_: HANDLE) -> HANDLE;
	pub fn GetDesktopWindow() -> HANDLE;
	pub fn GetDialogBaseUnits() -> i32;
	pub fn GetDlgCtrlID(_: HANDLE) -> i32;
	pub fn GetDlgItem(_: HANDLE, _: i32) -> HANDLE;
	pub fn GetDoubleClickTime() -> u32;
	pub fn GetFocus() -> HANDLE;
	pub fn GetForegroundWindow() -> HANDLE;
	pub fn GetGUIThreadInfo(_: u32, _: PVOID) -> BOOL;
	pub fn GetMenuInfo(_: HANDLE, _: PVOID) -> BOOL;
	pub fn GetMenuItemCount(_: HANDLE) -> i32;
	pub fn GetMenuItemID(_: HANDLE, _: i32) -> i32;
	pub fn GetMessageW(_: PVOID, _: HANDLE, _: u32, _: u32) -> BOOL;
	pub fn GetNextDlgGroupItem(_: HANDLE, _: HANDLE, _: BOOL) -> HANDLE;
	pub fn GetNextDlgTabItem(_: HANDLE, _: HANDLE, _: BOOL) -> HANDLE;
	pub fn GetParent(_: HANDLE) -> HANDLE;
	pub fn GetQueueStatus(_: u32) -> u32;
	pub fn GetScrollInfo(_: HANDLE, _: i32, _: PVOID) -> BOOL;
	pub fn GetScrollPos(_: HANDLE, _: i32) -> i32;
	pub fn GetSubMenu(_: HANDLE, _: i32) -> HANDLE;
	pub fn GetSysColor(_: i32) -> u32;
	pub fn GetSystemMetrics(_: i32) -> i32;
	pub fn GetUpdateRgn(_: HANDLE, _: HANDLE, _: BOOL) -> i32;
	pub fn GetWindow(_: HANDLE, _: u32) -> HANDLE;
	pub fn GetWindowDC(_: HANDLE) -> HANDLE;
	pub fn GetWindowDisplayAffinity(_: HANDLE, _: PVOID) -> BOOL;
	pub fn GetWindowInfo(_: HANDLE, _: PVOID) -> BOOL;
	pub fn GetWindowLongPtrW(_: HANDLE, _: i32) -> isize;
	pub fn GetWindowPlacement(_: HANDLE, _: PVOID) -> BOOL;
	pub fn GetWindowRect(_: HANDLE, _: PVOID) -> BOOL;
	pub fn GetWindowRgn(_: HANDLE, _: HANDLE) -> i32;
	pub fn GetWindowRgnBox(_: HANDLE, _: PVOID) -> i32;
	pub fn GetWindowTextLengthW(_: HANDLE) -> i32;
	pub fn GetWindowTextW(_: HANDLE, _: PSTR, _: i32) -> i32;
	pub fn GetWindowThreadProcessId(_: HANDLE, _: *mut u32) -> u32;
	pub fn HiliteMenuItem(_: HANDLE, _: HANDLE, _: u32, _: u32) -> BOOL;
	pub fn InsertMenuItemW(_: HANDLE, _: u32, _: BOOL, _: PCVOID) -> BOOL;
	pub fn InvalidateRect(_: HANDLE, _: PCVOID, _: BOOL) -> BOOL;
	pub fn InvalidateRgn(_: HANDLE, _: HANDLE, _: BOOL) -> BOOL;
	pub fn IsChild(_: HANDLE, _: HANDLE) -> BOOL;
	pub fn IsDialogMessageW(_: HANDLE, _: PVOID) -> BOOL;
	pub fn IsGUIThread(_: BOOL) -> BOOL;
	pub fn IsIconic(_: HANDLE) -> BOOL;
	pub fn IsMenu(_: HANDLE) -> BOOL;
	pub fn IsWindow(_: HANDLE) -> BOOL;
	pub fn IsWindowEnabled(_: HANDLE) -> BOOL;
	pub fn IsWindowUnicode(_: HANDLE) -> BOOL;
	pub fn IsWindowVisible(_: HANDLE) -> BOOL;
	pub fn IsZoomed(_: HANDLE) -> BOOL;
	pub fn KillTimer(_: HANDLE, _: usize) -> BOOL;
	pub fn LoadAcceleratorsW(_: HANDLE, _: PCSTR) -> HANDLE;
	pub fn LoadCursorW(_: HANDLE, _: PCSTR) -> HANDLE;
	pub fn LoadIconW(_: HANDLE, _: PCSTR) -> HANDLE;
	pub fn LoadImageW(_: HANDLE, _: PCSTR, _: u32, _: i32, _: i32, _: u32) -> HANDLE;
	pub fn LoadMenuW(_: HANDLE, _: PCSTR) -> HANDLE;
	pub fn LoadStringW(_: HANDLE, _: u32, _: PSTR, _: i32) -> i32;
	pub fn LockSetForegroundWindow(_: u32) -> BOOL;
	pub fn MapDialogRect(_: HANDLE, _: PVOID) -> BOOL;
	pub fn MessageBoxW(_: HANDLE, _: PCSTR, _: PCSTR, _: u32) -> i32;
	pub fn MoveWindow(_: HANDLE, _: i32, _: i32, _: i32, _: i32, _: BOOL) -> BOOL;
	pub fn OpenClipboard(_: HANDLE) -> BOOL;
	pub fn PeekMessageW(_: PVOID, _: HANDLE, _: u32, _: u32, _: u32) -> BOOL;
	pub fn PostMessageW(_: HANDLE, _: u32, _: usize, _: isize) -> BOOL;
	pub fn PostQuitMessage(_: i32);
	pub fn RealChildWindowFromPoint(_: HANDLE, _: i32, _: i32) -> HANDLE;
	pub fn RedrawWindow(_: HANDLE, _: PCVOID, _: HANDLE, _: u32) -> BOOL;
	pub fn RegisterClassExW(_: PCVOID) -> u16;
	pub fn ReleaseCapture() -> BOOL;
	pub fn ReleaseDC(_: HANDLE, _: HANDLE) -> i32;
	pub fn RemoveMenu(_: HANDLE, _: u32, _: u32) -> BOOL;
	pub fn ScreenToClient(_: HANDLE, _: PVOID) -> BOOL;
	pub fn SendMessageW(_: HANDLE, _: u32, _: usize, _: isize) -> isize;
	pub fn SetCapture(_: HANDLE) -> HANDLE;
	pub fn SetCaretBlinkTime(_: u32) -> BOOL;
	pub fn SetCaretPos(_: i32, _: i32) -> BOOL;
	pub fn SetClipboardData(_: u32, _: HANDLE) -> HANDLE;
	pub fn SetCursorPos(_: i32, _: i32) -> BOOL;
	pub fn SetFocus(_: HANDLE) -> HANDLE;
	pub fn SetForegroundWindow(_: HANDLE) -> BOOL;
	pub fn SetMenu(_: HANDLE, _: HANDLE) -> BOOL;
	pub fn SetMenuDefaultItem(_: HANDLE, _: u32, _: u32) -> BOOL;
	pub fn SetMenuInfo(_: HANDLE, _: PCVOID) -> BOOL;
	pub fn SetMenuItemInfoW(_: HANDLE, _: u32, _: BOOL, _: PCVOID) -> BOOL;
	pub fn SetParent(_: HANDLE, _: HANDLE) -> HANDLE;
	pub fn SetProcessDPIAware() -> BOOL;
	pub fn SetScrollInfo(_: HANDLE, _: i32, _: PCVOID, _: BOOL) -> i32;
	pub fn SetScrollPos(_: HANDLE, _: i32, _: i32, _: BOOL) -> i32;
	pub fn SetScrollRange(_: HANDLE, _: i32, _: i32, _: i32, _: BOOL) -> BOOL;
	pub fn SetSystemCursor(_: HANDLE, _: u32) -> BOOL;
	pub fn SetTimer(_: HANDLE, _: usize, _: u32, _: PFUNC) -> usize;
	pub fn SetWindowDisplayAffinity(_: HANDLE, _: u32) -> BOOL;
	pub fn SetWindowLongPtrW(_: HANDLE, _: i32, _: isize) -> isize;
	pub fn SetWindowPlacement(_: HANDLE, _: PCVOID) -> BOOL;
	pub fn SetWindowPos(_: HANDLE, _: HANDLE, _: i32, _: i32, _: i32, _: i32, _: u32) -> BOOL;
	pub fn SetWindowRgn(_: HANDLE, _: HANDLE, _: BOOL) -> i32;
	pub fn SetWindowsHookExW(_: i32, _: PFUNC, _: HANDLE, _: u32) -> HANDLE;
	pub fn SetWindowTextW(_: HANDLE, _: PCSTR) -> BOOL;
	pub fn ShowCaret(_: HANDLE) -> BOOL;
	pub fn ShowCursor(_: BOOL) -> i32;
	pub fn ShowWindow(_: HANDLE, _: i32) -> BOOL;
	pub fn SoundSentry() -> BOOL;
	pub fn SystemParametersInfoW(_: u32, _: u32, _: PVOID, _: u32) -> BOOL;
	pub fn TrackMouseEvent(_: PVOID) -> BOOL;
	pub fn TrackPopupMenu(_: HANDLE, _: u32, _: i32, _: i32, _: i32, _: HANDLE, _: PCVOID) -> BOOL;
	pub fn TranslateAcceleratorW(_: HANDLE, _: HANDLE, _: PVOID) -> i32;
	pub fn TranslateMessage(_: PCVOID) -> BOOL;
	pub fn UnhookWindowsHookEx(_: HANDLE) -> BOOL;
	pub fn UnregisterClassW(_: PCSTR, _: HANDLE) -> BOOL;
	pub fn UpdateWindow(_: HANDLE) -> BOOL;
	pub fn ValidateRect(_: HANDLE, _: PCVOID) -> BOOL;
	pub fn ValidateRgn(_: HANDLE, _: HANDLE) -> BOOL;
	pub fn WaitMessage() -> BOOL;
	pub fn WindowFromPhysicalPoint(_: i32, _: i32) -> HANDLE;
	pub fn WindowFromPoint(_: i32, _: i32) -> HANDLE;
	pub fn WinHelpW(_: HANDLE, _: PCSTR, _: u32, _: usize) -> BOOL;
}
