#[derive (Clone, Copy)]
pub enum UnityGfxRenderer
{
	OpenGL            =  0, // Desktop OpenGL 2 (deprecated)
	D3D9              =  1, // Direct3D 9
	D3D11             =  2, // Direct3D 11
	GCM               =  3, // PlayStation 3
	Null              =  4, // "null" device (used in batch mode)
	Xenon             =  6, // Xbox 360
	OpenGLES20        =  8, // OpenGL ES 2.0
	OpenGLES30        = 11, // OpenGL ES 3.x
	GXM               = 12, // PlayStation Vita
	PS4               = 13, // PlayStation 4
	XboxOne           = 14, // Xbox One        
	Metal             = 16, // iOS Metal
	OpenGLCore        = 17, // Desktop OpenGL core
	D3D12             = 18, // Direct3D 12
}

pub enum UnityGfxDeviceEventType {
    Initialize     = 0,
	Shutdown       = 1,
	BeforeReset    = 2,
	AfterReset     = 3,
}