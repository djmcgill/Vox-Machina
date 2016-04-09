/// Given a square heightmap image and a depth, turn it into a SVO

use svo::*;

use std::u8;

#[derive(Debug, PartialEq, Copy, Clone)]
struct SubImage<'a> {
	image: &'a[u8],
	image_width: u32,
	x_0: u32,
	x_n: u32,
	y_0: u32,
	y_n: u32,
	b_0: u8,
	b_n: u8
}

impl<'a> SubImage<'a> {
	pub fn new(image: &[u8], width: u32, height: u32) -> SubImage {
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

	fn rect(&self, x_0: u32, x_n: u32, y_0: u32, y_n: u32) -> SubImage<'a> {
		SubImage { x_0: x_0, x_n: x_n, y_0: y_0, y_n: y_n, .. *self }
	}

	fn quads(&self) -> Option<[SubImage<'a>; 4]> {
		let half_width = self.width() / 2;
		let half_height = self.height() / 2;

		guard!(half_width != 0 && half_height != 0);

		let ll = self.rect(self.x_0, self.x_0 + half_width, self.y_0, self.y_0 + half_height);
		let lr = self.rect(self.x_0, self.x_0 + half_width, self.y_0 + half_height, self.y_n);
		let rl = self.rect(self.x_0 + half_width, self.x_n, self.y_0, self.y_0 + half_height);
		let rr = self.rect(self.x_0 + half_width, self.x_n, self.y_0 + half_height, self.y_n);
		Some([ll, lr, rl, rr])
	}

	fn split_threshold(&self) -> Option<[SubImage<'a>; 2]> {
		let half_range = (self.b_n - self.b_0) / 2;
		guard!(half_range != 0);

		let lower = SubImage { b_0: self.b_0, b_n: self.b_0 + half_range, .. *self };
		let upper = SubImage { b_0: self.b_0 + half_range, b_n: self.b_n, .. *self };
		Some([lower, upper])
	}

	pub fn octs(&self) -> Option<[SubImage<'a>; 8]> {
		let layers = get!(self.split_threshold());
		let upper = get!(layers[0].quads());
		let lower = get!(layers[1].quads());
		Some([lower[0], lower[1], lower[2], lower[3],
		      upper[0], upper[1], upper[2], upper[3]])
	}

	pub fn byte_avg(&self) -> u8 {
		let sum: u32 = (self.y_0 .. self.y_n).map(|y| {
			(self.x_0 .. self.x_n).map(|x| {
				let ix = y*self.image_width + x;
				self.image[ix as usize] as u32
			}).fold(0u32, |x, y| x+y)
		}).fold(0u32, |x, y| x+y);
		(sum as usize / self.image.len()) as u8
	}
}

impl SVO {
	pub fn height_map(depth: u32, image: &[u8], width: u32, height: u32) -> SVO {
		assert_eq!(image.len(), (width * height) as usize);
		SVO::height_map_sub(depth, SubImage::new(image, width, height))
	}

	fn height_map_sub(depth: u32, image: SubImage) -> SVO {
		match image.octs() {
			Some(sub_images) if depth > 0 => // Recurs
				SVO::new_octants(|ix| SVO::height_map_sub(depth-1, sub_images[ix as usize])),

			_ => { // Make a voxel here
				let threshold = image.b_0 + (image.b_n - image.b_0) / 2;
				let voxel_type = if image.byte_avg() >= threshold { 1 } else { 0 };
				SVO::new_voxel(VoxelData::new(voxel_type), 0)
			}
		}
	}
}