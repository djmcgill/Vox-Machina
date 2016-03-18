using UnityEngine;
using System;
using System.Collections;
using System.Runtime.InteropServices;

public class NewBehaviourScript : MonoBehaviour
{
	new public Camera camera;
	public Transform test;
	public int numberOfObjects = 20;
	public float radius = 20f;

	SVO svo;

    void Start()
	{
        print("Starting");
		svo = new SVO();

		svo.SetBlock(new byte[] { 2 }, 0);
		svo.SetBlock(new byte[] { 3 }, 0);
		svo.SetBlock(new byte[] { 6 }, 0);
		svo.SetBlock(new byte[] { 7 }, 0);

		var orig = new Vector3 (3.268284f, 1.900771f, -9.700012f);
		var dir = new Vector3 (0f, 0f, 1f);
		print ("about to cast ray");
		var maybeHitPos = svo.CastRay (orig, dir);
		print ("succesfully cast ray");
		print (String.Format("ray returned {0}", maybeHitPos));

		SVO.OnBlocksCallback callback = (Vector3 vec, int depth, int voxelType) => {
			if (voxelType != 0)
			{
				print(String.Format("Vec: ( {0}, {1}, {2} ) depth: {3} type: {4}", vec.x, vec.y, vec.z, depth, voxelType));
				var obj = (Transform)Instantiate(test, vec, Quaternion.identity);
				float scale = (float) Math.Pow(2, -depth);
				obj.localScale = new Vector3(scale, scale, scale);
			}
		};

		svo.OnBlocks (callback);

        print("finished startup");
    }

	void Update()
	{
		if (svo == null) {print("svo was null"); return;}
//		Ray cameraRay = camera.ScreenPointToRay (Input.mousePosition);
//		print (String.Format ("calling castray with ({0}, {1}, {2}) ({3}, {4}, {5})", 
//			cameraRay.origin.x, cameraRay.origin.y, cameraRay.origin.z,
//			cameraRay.direction.x, cameraRay.direction.y, cameraRay.direction.z));
//		var maybeHitPos = svo.CastRayFloat (cameraRay.origin.x, cameraRay.origin.y, cameraRay.origin.z, 
//			cameraRay.direction.x, cameraRay.direction.y, cameraRay.direction.z);
//		print(String.Format("returned: {0}", maybeHitPos));
//
//		if (maybeHitPos.HasValue)
//		{
//			var hitPos = maybeHitPos.Value;
//			print (String.Format ("Ray hit at: ({0}, {1}, {2})", hitPos.x, hitPos.y, hitPos.z));
//		}
//			
	}
}
