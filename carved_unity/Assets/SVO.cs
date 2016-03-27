using System;
using System.Collections;
using System.Runtime.InteropServices;
using UnityEngine;

public class SVO : IDisposable
{
	private const int DEFAULT_BLOCK_TYPE = 1;
	private IntPtr svoPtr;
	private bool disposed = false;

	// Public interface
	public SVO()
	{
		rust_register_callback registerVoxel = (Vec3 vec, int depth, int voxelType) => {
			return 0;
		};

		rust_deregister_callback deregisterVoxel = (uint id) => {
			Console.WriteLine(String.Format("from unity deregistering {0}", id));
		};

		svoPtr = svo_create(DEFAULT_BLOCK_TYPE, registerVoxel, deregisterVoxel);
	}

	public SVO(int defaultVoxelType, UnityRegisterCallback registerVoxel, UnityDeregisterCallback deregisterVoxel)
	{
		rust_register_callback rustRegisterCallback = (Vec3 vec, int depth, int voxelType) => {
			return registerVoxel (RustToUnity (vec), depth, defaultVoxelType);
		};

		rust_deregister_callback rustDeregisterCallback = (uint id) => {
			Console.WriteLine(String.Format("from unity deregistering {0}", id));
			deregisterVoxel (id);
		};
			
		svoPtr = svo_create(defaultVoxelType, rustRegisterCallback, rustDeregisterCallback);
	}

	public void Dispose ()
	{
		Dispose(true);
		GC.SuppressFinalize(this);
	}

	private void Dispose(Boolean disposing)
	{
		if (disposed)
			return;
		svo_destroy (svoPtr);
		disposed = true;
	}

	~SVO()
	{
		Dispose(false);
	}

	/// Will return null if the ray misses.
	public Nullable<Vector3> CastRay (Vector3 rayOrigin, Vector3 rayDirection)
	{
		var rustOrigin = UnityToRust (rayOrigin);
		var rustDir = UnityToRust (rayDirection);
		var maybeHit = svo_cast_ray (svoPtr, rustOrigin, rustDir);

		if (maybeHit.isSome != 0)
		{
			return new Vector3 (maybeHit.x, maybeHit.y, maybeHit.z);
		}
		else 
		{
			return null;
		}

	}

	public void SetBlock (Byte[] index, int newBlockType)
	{
		svo_set_block (svoPtr, index, (UIntPtr)index.Length, newBlockType);
	}

	public delegate uint UnityRegisterCallback(Vector3 vec, int depth, int voxelType);
	private delegate uint rust_register_callback(Vec3 vec, int depth, int voxelType);
	public delegate void UnityDeregisterCallback(uint id);
	private delegate void rust_deregister_callback(uint id);

	// FFI interface
	[DllImport("libcarved_rust")]
	private static extern IntPtr svo_create (int voxelType, rust_register_callback register_voxel, rust_deregister_callback deregister_voxel);

	[DllImport("libcarved_rust")]
	private static extern void svo_destroy (IntPtr svo);

	[StructLayout(LayoutKind.Sequential)]
	private struct Vec3
	{
		public float x;
		public float y;
		public float z;
	}

	[StructLayout(LayoutKind.Sequential)]
	private struct BadOptionVec3
	{
		public int isSome;
		public float x;
		public float y;
		public float z;
	}

	[DllImport("libcarved_rust")]
	private static extern BadOptionVec3 svo_cast_ray (IntPtr svo, Vec3 rayOrigin, Vec3 rayDir);

	[DllImport("libcarved_rust")]
	private static extern void svo_set_block (IntPtr svo, Byte[] indexPtr, UIntPtr indexLen, int newBlockType);

	private Vector3 RustToUnity(Vec3 vec)
	{
		return new Vector3 (vec.x, vec.y, vec.z);
	}

	private Vec3 UnityToRust(Vector3 vec)
	{
		Vec3 ret;
		ret.x = vec.x;
		ret.y = vec.y;
		ret.z = vec.z;	
		return ret;
	}
}

