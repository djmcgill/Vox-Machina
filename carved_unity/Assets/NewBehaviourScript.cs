using UnityEngine;
using System;
using System.Collections;
using System.Runtime.InteropServices;

public class NewBehaviourScript : MonoBehaviour {
    [DllImport("libcarved_rust")]
    private static extern IntPtr svo_create(int x);

    [DllImport("libcarved_rust")]
    private static extern void svo_destroy(IntPtr svo);

    [DllImport("libcarved_rust")]
    private static extern int svo_get_voxel_type(IntPtr svo);

    [DllImport("libcarved_rust")]
    private static extern void svo_set_voxel_type(IntPtr svo, int block_type);

    [DllImport ("libcarved_rust")]
    private static extern void SetTimeFromUnity(float t);

    [DllImport ("libcarved_rust")]
    private static extern IntPtr GetRenderEventFunc();

    IEnumerator Start() {
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
        print("finished startup");
        yield return StartCoroutine("CallPluginAtEndOfFrames");
    }

    private IEnumerator CallPluginAtEndOfFrames() {
        while (true) {
            // Wait until all frame rendering is done
            yield return new WaitForEndOfFrame();

            // Set time for the plugin
            SetTimeFromUnity (Time.timeSinceLevelLoad);

            // Issue a plugin event with arbitrary integer identifier.
            // The plugin can distinguish between different
            // things it needs to do based on this ID.
            // For our simple plugin, it does not matter which ID we pass here.
            GL.IssuePluginEvent(GetRenderEventFunc(), 1);
        }
    }
}
