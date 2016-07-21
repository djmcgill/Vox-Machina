using UnityEngine;
using System;
using System.Collections;
using System.Runtime.InteropServices;


public class SVOController : MonoBehaviour
{
	new public Camera camera;
	public Transform test;
	public int numberOfObjects = 20;
	public float radius = 20f;

	private Hashtable drawnVoxels = new Hashtable ();
	private uint VOXEL_ID = 0;
	private object thisLock = new object();

	SVO svo;

    void Start()
	{
        print("Starting");


		SVO.UnityRegisterCallback registerCallback = (Vector3 vec, int depth, int voxelType) => {
			if (voxelType != 0)
			{
				lock(thisLock)
				{
					float scale = (float) Math.Pow(2, -depth);
					var adjustedVec = vec + new Vector3(scale/2, scale/2, scale/2);
					var obj = (Transform)Instantiate(test, adjustedVec, Quaternion.identity);

					obj.localScale = new Vector3(scale, scale, scale);
					var uid = VOXEL_ID++;
					drawnVoxels.Add(uid, obj);
					print(String.Format("Creating {5} Vec: ( {0}, {1}, {2} ) depth: {3} type: {4}", adjustedVec.x, adjustedVec.y, adjustedVec.z, depth, voxelType, uid));
					return uid;
				}
			}
			throw new ArgumentOutOfRangeException("Tried to draw a voxel with type 0");
		};

		SVO.UnityDeregisterCallback deregisterCallback = (uint id) => {
			print(String.Format("Deregistering {0}", id));
			lock(thisLock)
			{
				Transform voxel = (Transform)drawnVoxels[id];
				Destroy(voxel.gameObject);
				drawnVoxels.Remove(id);
			}
		};

		svo = new SVO(1, registerCallback, deregisterCallback);

		print ("Registered");

		svo.SetBlock(new byte[] { 2 }, 0);
		svo.SetBlock(new byte[] { 3 }, 0);
		svo.SetBlock(new byte[] { 6 }, 0);
		svo.SetBlock(new byte[] { 7 }, 0);


        print("finished startup");
    }

	void Update()
	{
		if (svo == null) {print("svo was null"); return;}
		Ray cameraRay = camera.ScreenPointToRay (Input.mousePosition);

		var maybeHitPos = svo.CastRay (cameraRay.origin, cameraRay.direction);

		if (maybeHitPos.HasValue)
		{
			var hitPos = maybeHitPos.Value;
			print (String.Format ("Ray hit at: ({0}, {1}, {2})", hitPos.x, hitPos.y, hitPos.z));

			if (Input.GetKeyDown (KeyCode.Q))
			{
				byte[] index = { };
				svo.SetBlock(index, 0);
			}
				
		}
			
	}
}
