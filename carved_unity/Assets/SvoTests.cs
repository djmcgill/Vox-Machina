using NUnit.Framework; // Exports a type called 'List'. What a great idea.
using System;
using System.Linq;
using g = System.Collections.Generic;
using UnityEngine;

[TestFixture]
public class SvoTests
{
	[Test]
	public void TestVoxel()
	{
		var svo = new SVO ();
		g.List<OnBlockResult> expected = new g.List<OnBlockResult> ();
		expected.Add(new OnBlockResult (new Vector3 (0f, 0f, 0f), 1, 1));
		SvoContains (svo, expected);
	}

	// Oh look, Unity's version of .net doesn't have the Tuple type.
	private class OnBlockResult
	{
		public Vector3 vec;
		public int depth;
		public int voxel_type;

		public OnBlockResult(Vector3 _vec, int _depth, int _voxel_type)
		{
			vec = _vec;
			depth = _depth;
			voxel_type = _voxel_type;
		}
	}


	private void SvoContains(SVO svo, g.IEnumerable<OnBlockResult> expected)
	{
		var results = new g.List<OnBlockResult>();
		SVO.OnBlocksCallback callback = (Vector3 vec, int depth, int voxelType) => {
			OnBlockResult result = new OnBlockResult (vec, depth, voxelType);
			results.Add (result);
		};
		svo.OnBlocks (callback);
		Assert.IsTrue(expected.SequenceEqual(results));
	}
}

