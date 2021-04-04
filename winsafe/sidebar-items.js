initSidebarItems({"enum":[["AtomStr","Variant parameter used in window class functions:"],["BitmapIcon","Variant parameter for:"],["BitmapPtrStr","Variant parameter used in menu methods:"],["BroadNull","Variant parameter for:"],["HwndHmenu","Variant parameter for:"],["HwndPlace","Variant parameter for:"],["HwndPointId","Variant parameter for:"],["IdIdcStr","Variant parameter for:"],["IdIdiStr","Variant parameter for:"],["IdMenu","Variant parameter used in menu methods:"],["IdPos","Variant parameter used in menu methods:"],["IdStr","Variant parameter for:"],["NccspRect","Variant parameter for:"],["RegistryValue","Variant value returned by `RegQueryValueEx`."],["WsWsex","Variant parameter for:"]],"fn":[["AdjustWindowRectEx","`AdjustWindowRectEx` function."],["CoCreateInstance","`CoCreateInstance` function."],["CoInitializeEx","`CoInitializeEx` function. Returns some error codes as success status."],["CoUninitialize","`CoUninitialize` function."],["DispatchMessage","`DispatchMessage` function."],["ExpandEnvironmentStrings","`ExpandEnvironmentStrings` function."],["FileTimeToSystemTime","`FileTimeToSystemTime` function."],["GetAsyncKeyState","`GetAsyncKeyState` function."],["GetDialogBaseUnits","`GetDialogBaseUnits` function."],["GetDoubleClickTime","`GetDoubleClickTime` function."],["GetEnvironmentStrings","`GetEnvironmentStrings` function."],["GetLastError","`GetLastError` function."],["GetMessage","`GetMessage` function."],["GetQueueStatus","`GetQueueStatus` function."],["GetSysColor","`GetSysColor` function."],["GetSystemMetrics","`GetSystemMetrics` function."],["GetSystemTime","`GetSystemTime` function."],["GetSystemTimeAsFileTime","`GetSystemTimeAsFileTime` function."],["GetSystemTimePreciseAsFileTime","`GetSystemTimePreciseAsFileTime` function."],["GetTickCount64","`GetTickCount64` function."],["HIBYTE","`HIBYTE` function. Originally a macro."],["HIDWORD","Returns the high-order `u32` of an `u64`."],["HIWORD","`HIWORD` function. Originally a macro."],["InitCommonControls","`InitCommonControls` function."],["IsGUIThread","`IsGUIThread` function."],["IsWindows10OrGreater","`IsWindows10OrGreater` function."],["IsWindows7OrGreater","`IsWindows7OrGreater` function."],["IsWindows8OrGreater","`IsWindows8OrGreater` function."],["IsWindows8Point1OrGreater","`IsWindows8Point1OrGreater` function."],["IsWindowsServer","`IsWindowsServer` function."],["IsWindowsVersionOrGreater","`IsWindowsVersionOrGreater` function."],["IsWindowsVistaOrGreater","`IsWindowsVistaOrGreater` function."],["LOBYTE","`LOBYTE` function. Originally a macro."],["LODWORD","Returns the low-order `u32` of an `u64`."],["LOWORD","`LOWORD` function. Originally a macro."],["LockSetForegroundWindow","`LockSetForegroundWindow` function."],["MAKEDWORD","Function that implements `MAKELONG`, `MAKEWPARAM`, and `MAKELPARAM` macros."],["MAKEWORD","`MAKEWORD` function. Originally a macro."],["MulDiv","`MulDiv` function."],["OutputDebugString","`OutputDebugString` function."],["PeekMessage","`PeekMessage` function."],["PostMessage","`PostMessage` function. Note that this function is asychronous."],["PostQuitMessage","`PostQuitMessage` function."],["RegisterClassEx","`RegisterClassEx` function."],["SetCaretBlinkTime","`SetCaretBlinkTime` function."],["SetCaretPos","`SetCaretPos` function."],["SetCursorPos","`SetCursorPos` function."],["SetLastError","`SetLastError` function."],["SetProcessDPIAware","`SetProcessDPIAware` function."],["ShowCursor","`ShowCursor` function."],["Sleep","`Sleep` function."],["SoundSentry","`SoundSentry` function."],["SystemParametersInfo","`SystemParametersInfo` function."],["SystemTimeToFileTime","`SystemTimeToFileTime` function."],["SystemTimeToTzSpecificLocalTime","`SystemTimeToTzSpecificLocalTime` function."],["TrackMouseEvent","`TrackMouseEvent` function."],["TranslateMessage","`TranslateMessage` function."],["UnregisterClass","`UnregisterClass` function."],["VerSetConditionMask","`VerSetConditionMask` function."],["VerifyVersionInfo","`VerifyVersionInfo` function."],["WaitMessage","`WaitMessage` function."]],"mod":[["co","Win32 constants and types of constants."],["gui","High-level GUI abstractions for user windows and native controls."],["msg","Parameters of window messages."],["shell","Shell COM interfaces."]],"struct":[["ACCEL","`ACCEL` struct."],["ATOM","`ATOM` returned by `RegisterClassEx`."],["BITMAPINFOHEADER","`BITMAPINFOHEADER` struct."],["BUTTON_IMAGELIST","`BUTTON_IMAGELIST` struct."],["BUTTON_SPLITINFO","`BUTTON_SPLITINFO` struct."],["CLSID","COM class ID. Just a safe abstraction over a `GUID`."],["COLORREF","`COLORREF` struct."],["CREATESTRUCT","`CREATESTRUCT` struct."],["DATETIMEPICKERINFO","`DATETIMEPICKERINFO` struct."],["FILETIME","`FILETIME` struct."],["GUID","`GUID` struct."],["HACCEL","Handle to an accelerator table."],["HBITMAP","Handle to a bitmap."],["HBRUSH","Handle to a brush."],["HCURSOR","Handle to a cursor."],["HDC","Handle to a device context."],["HDROP","Handle to an internal drop structure."],["HDWP","Handle to a deferred window position."],["HELPINFO","`HELPINFO` struct."],["HFONT","Handle to a font."],["HHOOK","Handle to a hook."],["HICON","Handle to an icon."],["HIMAGELIST","Handle to an image list."],["HINSTANCE","Handle to an instance, same as `HMODULE`."],["HKEY","Handle to a registry key."],["HLOCAL","Handle to a local memory block."],["HMENU","Handle to a menu."],["HPEN","Handle to a pen GDI object."],["HRGN","Handle to a region GDI object."],["HTHEME","Handle to a theme."],["HTREEITEM","Handle to an tree view item."],["HWND","Handle to a window."],["IID","COM interface ID. Just a safe abstraction over a `GUID`."],["IMAGELISTDRAWPARAMS","`IMAGELISTDRAWPARAMS` struct."],["IUnknown","`IUnknown` interface is the base to all COM interfaces."],["IUnknownVtbl","`IUnknownVtbl` is the base to all COM interface virtual tables."],["LITEM","`LITEM` struct."],["LOGFONT","`LOGFONT` struct."],["LVCOLUMN","`LVCOLUMN` struct."],["LVFINDINFO","`LVFINDINFO` struct."],["LVITEM","`LVITEM` struct."],["MENUINFO","`MENUINFO` struct."],["MENUITEMINFO","`MENUITEMINFO` struct."],["MINMAXINFO","`MINMAXINFO` struct."],["MSG","`MSG` struct."],["NCCALCSIZE_PARAMS","`NCCALCSIZE_PARAMS` struct."],["NMBCDROPDOWN","`NMBCDROPDOWN` struct."],["NMBCHOTITEM","`NMBCHOTITEM` struct."],["NMCHAR","`NMCHAR` struct."],["NMCUSTOMDRAW","`NMCUSTOMDRAW` struct."],["NMDATETIMECHANGE","`NMDATETIMECHANGE` struct."],["NMDATETIMEFORMAT","`NMDATETIMEFORMAT` struct."],["NMDATETIMEFORMATQUERY","`NMDATETIMEFORMATQUERY` struct."],["NMDATETIMESTRING","`NMDATETIMESTRING` struct."],["NMDATETIMEWMKEYDOWN","`NMDATETIMEWMKEYDOWN` struct."],["NMDAYSTATE","`NMDAYSTATE` struct."],["NMHDR","`NMHDR` struct."],["NMIPADDRESS","`NMIPADDRESS` struct."],["NMITEMACTIVATE","`NMITEMACTIVATE` struct."],["NMLINK","`NMLINK` struct."],["NMLISTVIEW","`NMLISTVIEW` struct."],["NMLVCACHEHINT","`NMLVCACHEHINT` struct."],["NMLVDISPINFO","`NMLVDISPINFO` struct."],["NMLVEMPTYMARKUP","`NMLVEMPTYMARKUP` struct."],["NMLVFINDITEM","`NMLVFINDITEM` struct."],["NMLVGETINFOTIP","`NMLVGETINFOTIP` struct."],["NMLVKEYDOWN","`NMLVKEYDOWN` struct."],["NMLVLINK","`NMLVLINK` struct."],["NMLVODSTATECHANGE","`NMLVODSTATECHANGE` struct."],["NMLVSCROLL","`NMLVSCROLL` struct."],["NMMOUSE","`NMMOUSE` struct."],["NMSELCHANGE","`NMSELCHANGE` struct."],["NMTVASYNCDRAW","`NMTVASYNCDRAW` struct."],["NMVIEWCHANGE","`NMVIEWCHANGE` struct."],["NONCLIENTMETRICS","`NONCLIENTMETRICS` struct."],["OSVERSIONINFOEX","`OSVERSIONINFOEX` struct."],["PAINTSTRUCT","`PAINTSTRUCT` struct."],["POINT","`POINT` struct."],["RECT","`RECT` struct."],["SCROLLINFO","`SCROLLINFO` struct."],["SECURITY_ATTRIBUTES","`SECURITY_ATTRIBUTES` struct."],["SIZE","`SIZE` struct."],["STYLESTRUCT_WS","`STYLESTRUCT` struct for `WS`."],["STYLESTRUCT_WS_EX","`STYLESTRUCT` struct for `WS_EX`."],["SYSTEMTIME","`SYSTEMTIME` struct."],["TEXTMETRIC","`TEXTMETRIC` struct."],["TIME_ZONE_INFORMATION","`TIME_ZONE_INFORMATION` struct."],["TRACKMOUSEEVENT","`TRACKMOUSEEVENT` struct."],["WINDOWINFO","`WINDOWINFO` struct."],["WINDOWPLACEMENT","`WINDOWPLACEMENT` struct."],["WINDOWPOS","`WINDOWPOS` struct."],["WNDCLASSEX","`WNDCLASSEX` struct."],["WString","Stores a `Vec<u16>` buffer for an Unicode UTF-16 wide string natively used by Windows."]],"trait":[["Vtbl","Trait for any COM virtual table."]],"type":[["DLGPROC","Type alias to `DLGPROC` callback function."],["HOOKPROC","Type alias to `HOOKPROC` callback function."],["PPVtbl","Type alias to pointer to pointer to any COM virtual table."],["SUBCLASSPROC","Type alias to `SUBCLASSPROC` callback function."],["TIMERPROC","Type alias to `TIMERPROC` callback function."],["WNDENUMPROC","Type alias to `WNDENUMPROC` callbak function."],["WNDPROC","Type alias to `WNDPROC` callback function."],["WinResult","A specialized `Result` for Win32 operations, which return a `ERROR` on failure."]]});