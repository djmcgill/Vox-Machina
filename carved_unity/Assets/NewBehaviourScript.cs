using UnityEngine;
using System;
using System.Collections;
using System.Runtime.InteropServices;

public class NewBehaviourScript : MonoBehaviour {
    [DllImport("carved_rust")]
    private static extern IntPtr svo_create(int x);

    [DllImport("carved_rust")]
    private static extern void svo_destroy(IntPtr svo);

    [DllImport("carved_rust")]
    private static extern int svo_get_voxel_type(IntPtr svo);

    [DllImport("carved_rust")]
    private static extern void svo_set_voxel_type(IntPtr svo, int block_type);

    // Use this for initialization
    void Start () {
        print("Starting");
        var svo = svo_create(1);
        var block1 = svo_get_voxel_type(svo);
        print(block1);
        var block2 = svo_get_voxel_type(svo);
        print(block2);
        svo_set_voxel_type(svo, 2);
        var block3 = svo_get_voxel_type(svo);
        print(block3);
        svo_destroy(svo);
        print("finished");
    }
	
	// Update is called once per frame
	void Update () {
	
	}
}
