use unity_interface_guid::*;
use unity_interface_enums::*;

use std::cell::Cell;
use std::sync::Mutex;

lazy_static! {
	static ref PLUGIN: Mutex<Cell<Option<UnityPlugin<'static>>>> = Mutex::new(Cell::new(None));
}

fn plugin_set(new_plugin: Option<UnityPlugin<'static>>) {
	PLUGIN.lock().unwrap().set(new_plugin);
}

fn plugin_get() -> Option<UnityPlugin<'static>> {
	PLUGIN.lock().unwrap().get()
}

#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn UnityPluginLoad(unity_interfaces: *const IUnityInterfaces) {
	println!("UnityPluginLoad called with {:?}", unity_interfaces);
	let plugin : UnityPlugin = unsafe { UnityPlugin::new(&*unity_interfaces) };
	plugin_set(Some(plugin));
	on_graphics_device_event(UnityGfxDeviceEventType::Initialize);
}

#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn UnityPluginUnload() {
	let plugin = plugin_get().unwrap();
	plugin.graphics_interface.unregister_device_event_callback(on_graphics_device_event);
	println!("1");
	plugin_set(None);
	println!("2");
}

#[derive (Copy, Clone)]
pub struct UnityPlugin<'a> {
	unity_interfaces   : &'a IUnityInterfaces,
	graphics_interface : &'a IUnityGraphics,
	graphics_renderer  : Option<UnityGfxRenderer>,
}

impl<'a> UnityPlugin<'a> {
	fn new(unity_interfaces: &'a IUnityInterfaces) -> UnityPlugin<'a> {
		UnityPlugin {
			unity_interfaces: unity_interfaces,
			graphics_interface: unity_interfaces.get_graphics_interface(),
			graphics_renderer: None
		}
	}
}

#[repr(C)]
pub struct IUnityInterfaces {
	// Returns an interface matching the guid.
	// Returns nullptr if the given interface is unavailable in the active Unity runtime.
	get_interface_raw : fn(UnityInterfaceGUID) -> *mut IUnityInterface,

	// Registers a new interface.	
	register_interface_raw : fn(UnityInterfaceGUID, *mut IUnityInterface) -> ()
}

impl IUnityInterfaces {
	fn get_interface(&self, guid : UnityInterfaceGUID) -> Option<*mut IUnityInterface> {
		let interface_ptr : *mut IUnityInterface = (self.get_interface_raw)(guid);
		if interface_ptr.is_null() {None} else {Some(interface_ptr)}
	}

	fn get_graphics_interface<'a>(&self) -> &'a IUnityGraphics {
		println!("get graphics interface");
		let interface_ptr = self.get_interface(UnityInterfaceGUID::graphics_guid()).unwrap();
		let graphics_ref = unsafe { &mut *(interface_ptr as *mut IUnityGraphics)};
		graphics_ref.register_device_event_callback(on_graphics_device_event);
		println!("finished get graphics interface");
		graphics_ref
	}
}

struct IUnityInterface;

#[repr(C)]
struct IUnityGraphics {
	get_renderer_raw : fn() -> UnityGfxRenderer, // Thread safe

	// This callback will be called when graphics device is created, destroyed, reset, etc.
	// It is possible to miss the Initialize event in case plugin is loaded at a later time,
	// when the graphics device is already created.
	register_device_event_callback_raw   : fn (fn (UnityGfxDeviceEventType) -> ()) -> (),
	unregister_device_event_callback_raw : fn (fn (UnityGfxDeviceEventType) -> ()) -> ()
}

impl IUnityGraphics {
	fn get_renderer(&self) -> UnityGfxRenderer {
		(self.get_renderer_raw)()
	}

	fn register_device_event_callback(&self, callback: fn (UnityGfxDeviceEventType) -> ()) {
		println!("registering callback");
		(self.register_device_event_callback_raw)(callback)
	}

	fn unregister_device_event_callback(&self, callback: fn (UnityGfxDeviceEventType) -> ()) {
		println!("unregistering callback");
		(self.unregister_device_event_callback_raw)(callback)	
	}
}

// This is the function that actually does stuff.
fn on_graphics_device_event(event_type: UnityGfxDeviceEventType) {
	println!("on_graphics_device_event called with {:?}", event_type);
	match event_type {
		UnityGfxDeviceEventType::Initialize => {
			let cell = PLUGIN.lock().unwrap();
			let old_plugin = cell.get().unwrap();
			let new_renderer = old_plugin.graphics_interface.get_renderer();
			cell.set(Some(UnityPlugin {graphics_renderer: Some(new_renderer), .. old_plugin}));
            // TODO: user initialization code
		},
		UnityGfxDeviceEventType::Shutdown => {
			let cell = PLUGIN.lock().unwrap();
			let old_plugin = cell.get().unwrap();
			cell.set(Some(UnityPlugin {graphics_renderer: None, .. old_plugin}));
            //TODO: user shutdown code
		},
		UnityGfxDeviceEventType::BeforeReset => {
			//TODO: user Direct3D 9 code
		},
		UnityGfxDeviceEventType::AfterReset => {
			//TODO: user Direct3D 9 code
		},
	}
	println!("on_graphics_device_event called with {:?} finished", event_type);
}

#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn SetTimeFromUnity(time: f32) {
	println!("SetTimeFromUnity with {:?}", time);
}

#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn GetRenderEventFunc() -> (fn(i32) -> ()) {
	println!("GetRenderEventFunc called");
	on_render_event
}

#[allow(non_snake_case)]
fn on_render_event(eventID: i32) {
	println!("on_render_event called with {}", eventID);
}
