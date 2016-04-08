/// Given a square heightmap image and a depth, turn it into a SVO

use svo::*;

use std::u8;

#[derive(Debug, PartialEq, Copy, Clone)]
struct SubImage<'a> {
	image: &'a Box<[u8]>,
	image_width: u32,
	x_0: u32,
	x_n: u32,
	y_0: u32,
	y_n: u32,
	b_0: u8,
	b_n: u8
}

impl<'a> SubImage<'a> {
	pub fn new(image: &Box<[u8]>, width: u32, height: u32) -> SubImage {
		assert_eq!(image.len(), (width * height) as usize);
		SubImage {
			image: image, image_width: width,
		    x_0: 0, x_n: width, y_0: 0, y_n: height,
		    b_0: 0, b_n: u8::MAX
		}
	}

	pub fn width(&self) -> u32 {
		self.x_n - self.x_0
	}

	pub fn height(&self) -> u32 {
		self.y_n - self.y_0
	}

	fn rect(&self, x_0: u32, x_n: u32, y_0: u32, y_n: u32) -> SubImage {
		SubImage { x_0: x_0, x_n: x_n, y_0: y_0, y_n: y_n, .. *self }
	}

	fn quads(&'a self) -> Option<[SubImage<'a>; 4]> {
		let half_width = self.width() / 2;
		let half_height = self.height() / 2;

		if half_width == 0 || half_height == 0 { return None; }

		let ll = self.rect(self.x_0, self.x_0 + half_width, self.y_0, self.y_0 + half_height);
		let lr = self.rect(self.x_0, self.x_0 + half_width, self.y_0 + half_height, self.y_n);
		let rl = self.rect(self.x_0 + half_width, self.x_n, self.y_0, self.y_0 + half_height);
		let rr = self.rect(self.x_0 + half_width, self.x_n, self.y_0 + half_height, self.y_n);
		Some([ll, lr, rl, rr])
	}

	fn split_threshold(&'a self) -> Option<[SubImage<'a>; 2]> {
		let half_range = (self.b_n - self.b_0) / 2;
		if half_range == 0 { return None; }

		let lower = SubImage { b_0: self.b_0, b_n: self.b_0 + half_range, .. *self };
		let upper = SubImage { b_0: self.b_0 + half_range, b_n: self.b_n, .. *self };
		Some([lower, upper])
	}

	pub fn octs(&'a self) -> Option<[SubImage<'a>; 8]> {
		// error: `quads[..]` does not live long enough
		//note: reference must be valid for the lifetime 'a as defined on the block at 63:52...
		//...but borrowed value is only valid for the scope of parameters for function at 64:51
		self.quads().and_then(|quads: [SubImage<'a>; 4]| {
			let octs01 = quads[0].split_threshold().unwrap();
			let octs23 = quads[1].split_threshold().unwrap();
			let octs45 = quads[2].split_threshold().unwrap();
			let octs67 = quads[3].split_threshold().unwrap();
			let octs0: SubImage<'a> = octs01[0];
			let octs1: SubImage<'a> = octs01[1];
			let octs2: SubImage<'a> = octs23[0];
			let octs3: SubImage<'a> = octs23[1];
			let octs4: SubImage<'a> = octs45[0];
			let octs5: SubImage<'a> = octs45[1];
			let octs6: SubImage<'a> = octs67[0];
			let octs7: SubImage<'a> = octs67[1];
			let octs_all: [SubImage<'a>; 8] = [octs0, octs1, octs2, octs3, octs4, octs5, octs6, octs7];
			Some(octs_all)
		})
	}

	pub fn byte_avg(&self) -> u8 {
		let sum: u32 = (self.y_0 .. self.y_n).map(|y| {
			(self.x_0 .. self.x_n).map(|x| {
				let ix = y*self.image_width + x;
				self.image[ix as usize] as u32
			}).fold(0u32, |x, y| x+y) // Dear Rust, fuck you.
		}).fold(0u32, |x, y| x+y);
		(sum as usize / self.image.len()) as u8
	}
}

impl SVO {
	pub fn height_map(depth: u32, image: &[u8], width: u32, height: u32) -> SVO {
		assert_eq!(image.len(), (width * height) as usize);
		//SVO::height_map_sub(depth, image, 0, width-1, 0, height-1, (u8::MAX / 2) as u16)
		panic!()
	}

	fn height_map_sub(depth: u32, image: SubImage) -> SVO {
		if depth == 0 {
			// Make a voxel
			//let sub_image_ixs = panic!() //: Iterator<Item=u32> = (y_0..y_n).flatMap(|y| (x_0..x_n).map(|x| )).collect();
			let voxel_type = panic!(); //if byte_avg(image, sub_image_ixs) as u16 >= pixel_threshold { 1 } else { 0 };
			SVO::new_voxel(VoxelData::new(voxel_type), 0)
		} else {
			let octants = image.quads();

			panic!()
			//SVO::Octants([])
		}
	}
}