using UnityEngine;
using System.Collections;

public class CameraTarget : MonoBehaviour {
	public float XPanMult = 10f;
	public float YPanMult = 10f;
	public float ZPanMult = 10f;
	public new Camera camera;
	public float DefaultCameraDistance;
	public float VTiltMult;
	public float HTiltMult;

	private float cameraDistance = 1;

	// Use this for initialization
	void Start () {
		cameraDistance = DefaultCameraDistance;
		transform.position = 0.5f * Vector3.one;
		camera.transform.localPosition = cameraDistance * Vector3.forward;
		camera.transform.LookAt (transform);
	}

	// Update is called once per frame
	void Update () {
		var dt = Time.deltaTime;

		if (Input.GetMouseButtonDown (0)) {
			Cursor.lockState = CursorLockMode.Locked;
			Cursor.visible = false;
		}

		if (Input.GetMouseButtonUp (0)) {
			Cursor.lockState = CursorLockMode.None;
			Cursor.visible = true;
		}

		if (Input.GetMouseButton (0)) {
			// Calculate the new position
			var xDiff = Input.GetAxis("Mouse X");
			var yDiff = Input.GetAxis("Mouse Y");

			if (xDiff * yDiff != 0f) {
				var vDiff = yDiff * VTiltMult * dt;
				var hDiff = xDiff * HTiltMult * dt;
				camera.transform.RotateAround(transform.position, Vector3.up, hDiff);
				var localLeft = camera.transform.TransformDirection (Vector3.left);
				camera.transform.RotateAround(transform.position, localLeft, vDiff);
			}

		} else {
			// Calculate the transform
			var movement = new Vector3 ();

			if (Input.GetKey (KeyCode.UpArrow)) {
				var localForward = camera.transform.TransformDirection (Vector3.forward);
				movement += dt * ZPanMult * new Vector3(localForward.x, 0, localForward.z);
			}

			if (Input.GetKey (KeyCode.DownArrow)) {
				var localBack = camera.transform.TransformDirection (Vector3.back);
				movement += dt * ZPanMult * new Vector3(localBack.x, 0, localBack.z);
			}

			if (Input.GetKey (KeyCode.LeftArrow)) {
				var localLeft = camera.transform.TransformDirection (Vector3.left);
				movement += dt * XPanMult * localLeft;
			}

			if (Input.GetKey (KeyCode.RightArrow)) {
				var localRight = camera.transform.TransformDirection (Vector3.right);
				movement += dt * XPanMult * localRight;
			}

			if (Input.GetKey (KeyCode.Period)) {
				movement += dt * YPanMult * Vector3.up;
			}

			if (Input.GetKey (KeyCode.Comma)) {
				movement += dt * YPanMult * Vector3.down;
			}

			// Actually transform
			if (!movement.Equals(Vector3.zero)) {
				transform.Translate(movement);
				camera.transform.Translate(movement, transform); // Move the camera the same direction as the target.
			}
		}
	}
}
