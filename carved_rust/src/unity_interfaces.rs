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
	let plugin : UnityPlugin = unsafe {UnityPlugin::new(&*unity_interfaces) };
	plugin_set(Some(plugin));
}

#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn UnityPluginUnload() {
	plugin_set(None);
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
		let interface_ptr = self.get_interface(UnityInterfaceGUID::graphics_guid()).unwrap();
		let graphics_ref = unsafe { &mut *(interface_ptr as *mut IUnityGraphics)};
		graphics_ref.register_device_event_callback(on_graphics_device_event);
		on_graphics_device_event(UnityGfxDeviceEventType::Initialize);
		graphics_ref
	}
}

struct IUnityInterface;

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

	fn register_device_event_callback(&mut self, callback: fn (UnityGfxDeviceEventType) -> ()) {
		(self.register_device_event_callback_raw)(callback)
	}

	fn unregister_device_event_callback(&mut self, callback: fn (UnityGfxDeviceEventType) -> ()) {
		(self.unregister_device_event_callback_raw)(callback)	
	}
}

impl Drop for IUnityGraphics {
	fn drop(&mut self) {
		self.unregister_device_event_callback(on_graphics_device_event);
	}
}

// This is the function that actually does stuff.
fn on_graphics_device_event(event_type: UnityGfxDeviceEventType) {
	match event_type {
		UnityGfxDeviceEventType::Initialize => {
			let cell = PLUGIN.lock().unwrap();
			let old_plugin = plugin_get().unwrap();
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
}
