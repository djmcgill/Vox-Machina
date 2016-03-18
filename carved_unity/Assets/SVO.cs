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
		svoPtr = svo_create(DEFAULT_BLOCK_TYPE);
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

	public delegate void OnBlocksCallback(Vector3 vec, int depth, int voxelType);

	public void OnBlocks (OnBlocksCallback onBlocks)
	{
		RustOnBlocksCallback rustOnBlocks = (Vec3 vec, int depth, int voxelType) => {
			var vector = new Vector3 (vec.x, vec.y, vec.z);
			onBlocks (vector, depth, voxelType);
		};
		svo_on_voxels(svoPtr, rustOnBlocks);
	}

	/// Will return null if the ray misses.
	public Nullable<Vector3> CastRay (Vector3 rayOrigin, Vector3 rayDirection)
	{
		Vec3 rustOrigin;
		rustOrigin.x = rayOrigin.x;
		rustOrigin.y = rayOrigin.y;
		rustOrigin.z = rayOrigin.z;

		Vec3 rustDir;
		rustDir.x = rayDirection.x;
		rustDir.y = rayDirection.y;
		rustDir.z = rayDirection.z;
			
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


	// FFI interface
	[DllImport("libcarved_rust")]
	private static extern IntPtr svo_create (int x);

	[DllImport("libcarved_rust")]
	private static extern void svo_destroy (IntPtr svo);

	private delegate void RustOnBlocksCallback (Vec3 vec, int depth, int voxel_type);

	[DllImport("libcarved_rust")]
	private static extern void svo_on_voxels (IntPtr svo, RustOnBlocksCallback onBlocks);

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
}

