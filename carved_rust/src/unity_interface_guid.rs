pub struct UnityInterfaceGUID {
	high: u64, // TODO: use c_ulonglong instead
	low: u64
}

impl UnityInterfaceGUID {
	pub fn graphics_guid() -> UnityInterfaceGUID {
		UnityInterfaceGUID { high: 0x7CBA0A9CA4DDB544, low: 0x8C5AD4926EB17B11}
	}
}