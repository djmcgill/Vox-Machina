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

	SVO svo;

    void Start()
	{
        print("Starting");

		SVO.UnityRegisterCallback registerCallback = (Vector3 vec, int depth, int voxelType) => {
			if (voxelType != 0)
			{
				print(String.Format("Vec: ( {0}, {1}, {2} ) depth: {3} type: {4}", vec.x, vec.y, vec.z, depth, voxelType));
				var obj = (Transform)Instantiate(test, vec, Quaternion.identity);
				float scale = (float) Math.Pow(2, -depth);
				obj.localScale = new Vector3(scale, scale, scale);
			}
			return 0;
		};

		SVO.UnityDeregisterCallback deregisterCallback = (uint id) => {};

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
		}
			
	}
}
