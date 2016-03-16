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
		// Dispose of unmanaged resources.
		Dispose(true);
		// Suppress finalization.
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
		Console.WriteLine (String.Format ("about to call cast ray on {2} with {0} {1}", rayOrigin, rayDirection, svoPtr));
		var rustOrigin = new Vec3 (rayOrigin.x, rayOrigin.y, rayOrigin.z);
		var rustDir = new Vec3 (rayDirection.x, rayDirection.y, rayDirection.z);
		Console.WriteLine ("really calling it");
		var maybeHit = svo_cast_ray (svoPtr, rustOrigin, rustDir);
		Console.WriteLine (String.Format ("maybeHit {0}", maybeHit));
		if (maybeHit.isSome) 
		{
			var vec = maybeHit.value;
			return new Vector3 (vec.x, vec.y, vec.z);
		}
		else
		{
			return null;
		}
	}
	//private static extern BadOptionVec3 svo_cast_ray_float (IntPtr svo, float ox, float oy, float oz, float dx, float dy, float dz);
	public Nullable<Vector3> CastRayFloat (float ox, float oy, float oz, float dx, float dy, float dz)
	{
		Console.WriteLine ("really calling float version of it");
		Console.WriteLine ("{0} {1} {2} {3} {4} {5} {6}", svoPtr, ox, oy, oz, dx, dy, dz);
		var maybeHit = svo_cast_ray_float (svoPtr, (int)ox, (int)oy, (int)oz, (int)dx, (int)dy, (int)dz);
		Console.WriteLine (String.Format ("maybeHit {0}", maybeHit));
		if (maybeHit.isSome) 
		{
			var vec = maybeHit.value;
			return new Vector3 (vec.x, vec.y, vec.z);
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
		public float x, y, z;

		public Vec3 (float _x, float _y, float _z)
		{
			x = _x;
			y = _y;
			z = _z;
		}
	}

	[StructLayout(LayoutKind.Sequential)]
	private struct BadOptionVec3
	{
		public bool isSome;
		public Vec3 value;
	}

	[DllImport("libcarved_rust")]
	private static extern BadOptionVec3 svo_cast_ray (IntPtr svo, Vec3 rayOrigin, Vec3 rayDir);

	[DllImport("libcarved_rust")]
	private static extern BadOptionVec3 svo_cast_ray_float (IntPtr svo, int ox, int oy, int oz, int dx, int dy, int dz);



	[DllImport("libcarved_rust")]
	private static extern void svo_set_block (IntPtr svo, Byte[] indexPtr, UIntPtr indexLen, int newBlockType);
}

