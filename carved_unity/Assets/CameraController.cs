using UnityEngine;
using System.Collections;

public class CameraController : MonoBehaviour {
	public float hMult = 10f;
	public float vMult = 10f;

	// Use this for initialization
	void Start () {
	
	}
	
	// Update is called once per frame
	void Update () {
		var hMove = hMult * Input.GetAxis ("Mouse X");
		var vMove = vMult * Input.GetAxis ("Mouse Y");
		transform.Translate (vMove, hMove, 0);
	
	}
}
