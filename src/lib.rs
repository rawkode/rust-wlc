#![feature(libc)]
// This code will be used
#![allow(dead_code)]
extern crate libc;

#[test]
fn it_works() {
}

// Types

enum LogType {
    Info,
    Warn,
    Error,
    Wayland
}

enum BackendType {
    None,
    DRM,
    X11
}

enum EventBit {
    Readable = 1,
    Writeable = 2,
    Hangup = 4,
    Error = 8
}

enum ViewState {
    Maximized = 1,
    Fullscreen = 2,
    Resizing = 4,
    Moving = 8,
    Activated = 16
}

enum ViewType {
    /// Override redirect (X11)
    OverrideRedirect = 1,
    /// Tooltips (X11)
    Unmanaged = 2,
    /// Splash Screens (X11)
    Splash = 4,
    /// Modal Windows (X11)
    Modal = 8,
    /// xdg-shell, wl-shell popups
    Popup = 16
}

enum ResizeEdge {
    None = 0,
    Top = 1,
    Bottom = 2,
    Left = 4,
    TopLeft = 5,
    BottomLeft = 6,
    Right = 8,
    TopRight = 9,
    BottomRight = 10
}

/// Represents which keyboard meta keys are being pressed.
enum KeyModifier {
    None = 0,
    Shift = 1,
    Caps = 2,
    Ctrl = 4,
    Alt = 8,
    Mod2 = 16,
    Mod3 = 32,
    Logo = 64,
    Mod5 = 128
}

/// "LEDs" or active key-locks.
/// i.e. caps lock, scroll lock
enum KeyboardLed {
    None = 0,
    NumLock = 1,
    CapsLock = 2,
    ScrollLock = 4
}

enum KeyState {
    Released = 0,
    Pressed = 1
}

enum ButtonState {
    Released = 0,
    Pressed = 1
}

/// Which axis of the scroll wheel is being used
enum ScrollAxis {
    None = 0,
    Vertical = 1,
    Horizontal = 2
}

/// Touch type in touch interface handler
enum TouchType {
    Down,
    Up,
    Motion,
    Frame,
    Cancel
}

/// State of keyoard modifiers.
/// i.e. control key, caps lock on
#[repr(C)]
struct KeyboardModifiers {
    leds: KeyboardLed,
    mods: KeyModifier
}

#[repr(C)]
struct Point {
    x: i32,
    y: i32
}

#[repr(C)]
struct WLCSize {
    w: i32,
    h: i32
}

#[repr(C)]
struct Geometry {
    size: WLCSize,
    origin: Point
}

type InterfaceHandler = fn(WLCHandle) -> ();

type WLCHandle = libc::uintptr_t;

#[repr(C)]
struct WlcInterface {
    output: OutputInterface,
    view: ViewInterface,
    keyboard: KeyboardInterface,
    pointer: PointerInterface,
    touch: TouchInterface,
    compositor: CompositorInterface,
    input: InputInterface
}

#[repr(C)]
struct OutputInterface {
    created: fn(WLCHandle) -> bool,
    destroyed: InterfaceHandler,
    focus: fn(WLCHandle, bool) -> (),
    resolution: fn(WLCHandle, WLCSize, WLCSize) -> (),
    render: RenderInterface,
}

#[repr(C)]
struct RenderInterface {
    pre: InterfaceHandler,
    post: InterfaceHandler,
}

#[repr(C)]
struct ViewInterface {
    created: fn(WLCHandle) -> bool,
    destroyed: InterfaceHandler,
    focus: fn(WLCHandle, bool) -> (),
    move_to_output: fn(WLCHandle, WLCHandle, WLCHandle) -> (),
    request: RequestInterface,
}

#[repr(C)]
struct RequestInterface {
    geometry: fn(WLCHandle, Geometry) -> (),
    state: fn(WLCHandle, ViewState, bool) -> (),
    move_: fn(WLCHandle, Point) -> (),
    resize: fn(WLCHandle, ResizeEdge, Point) -> (),
    render: RenderInterface,
}

#[repr(C)]
struct KeyboardInterface {
    key: fn(WLCHandle, u32, KeyboardModifiers, u32, KeyState) -> bool,
}

#[repr(C)]
struct PointerInterface {
    button: fn(WLCHandle, u32, KeyboardModifiers, u32, ButtonState, Point) -> bool,
    scroll: fn(WLCHandle, u32, KeyboardModifiers, ScrollAxis, [u64; 2]) -> bool,
    motion: fn(WLCHandle, u32, Point),
}

#[repr(C)]
struct TouchInterface {
    touch: fn(WLCHandle, u32, KeyboardModifiers, TouchType, i32, Point) -> bool,
}

#[repr(C)]
struct CompositorInterface {
    ready: fn() -> ()
}

#[repr(C)]
struct InputInterface {
    created: fn(LibinputDevice) -> bool,
    destroyed: fn(LibinputDevice) -> ()
}

/// Not currently supporting libinput
//#[repr(C)]
enum LibinputDevice {}

//fn wlc_set_log_handler()
