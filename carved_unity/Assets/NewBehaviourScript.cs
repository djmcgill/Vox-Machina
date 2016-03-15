using UnityEngine;
using System;
using System.Collections;
using System.Runtime.InteropServices;

public class NewBehaviourScript : MonoBehaviour
{

	public Transform test;
	public int numberOfObjects = 20;
	public float radius = 20f;

    void Start()
	{
        print("Starting");
		var svo = new SVO();

		SVO.OnBlocksCallback callback = (Vector3 vec, int depth, int voxelType) => {
			print(vec);
			print(depth);
			print(voxelType);
		};

		svo.OnBlocks (callback);

		for (int i = 0; i < numberOfObjects; i++) {
			float angle = i * Mathf.PI * 2 / numberOfObjects;
			Vector3 pos = new Vector3(Mathf.Cos(angle), 0, Mathf.Sin(angle)) * radius;
			Instantiate(test, pos, Quaternion.identity);
		}

        print("finished startup");


    }
}
