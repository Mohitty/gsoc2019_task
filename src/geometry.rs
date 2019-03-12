extern crate nalgebra;

use nalgebra::{Vector3, Matrix3, Real};

pub struct Plane {
	pub plane_center: Vector3<f32>,
	pub rot_matrix: Matrix3<f32>,
	pub width: f32,
	pub height: f32
}

impl Plane {
	pub fn new(center: Vector3<f32>, diru: Vector3<f32>, dirv: Vector3<f32>, width: f32, height: f32) -> Plane {
		let x = diru.y * dirv.z - diru.z * dirv.y;
		let y = diru.z * dirv.x - diru.x * dirv.z;
		let z = diru.x * dirv.y - diru.y * dirv.x;

		let mut dirz = Vector3::new(x,y,z);
		dirz.normalize_mut();

		let m = Matrix3::from_columns(&[diru, dirv,dirz]);

		Plane {
			plane_center: center,
			rot_matrix: m,
			width: width,
			height: height,
		}
	}

	pub fn local_to_global(&self, point: Vector3<f32>) -> Vector3<f32> {
		self.plane_center + self.rot_matrix*point
	}

	pub fn global_to_local(&self, point: Vector3<f32>) -> Vector3<f32> {
		let inv = inversematrix(&self.rot_matrix);
		inv*(point-self.plane_center)
	}

	pub fn is_inside(&self, point: Vector3<f32>) -> bool {
		let local_point = self.global_to_local(point);
		if local_point.z == 0.0 && Real::abs(local_point.x) <= (self.width/2.0) && Real::abs(local_point.y) <= (self.height/2.0) {
			return true;
		} else {
			return false;
		}
	}
}

pub fn inversematrix(m: &Matrix3<f32>) -> Matrix3<f32> {

	let det: f32 = m[(0,0)]*(m[(1,1)]*m[(2,2)]-m[(2,1)]*m[(1,2)]) -
				  m[(0,1)]*(m[(1,0)]*m[(2,2)]-m[(2,0)]*m[(1,2)]) +
				  m[(0,2)]*(m[(1,0)]*m[(2,1)]-m[(2,0)]*m[(1,1)]);

	let mut r1 = det*(m[(1,1)]*m[(2,2)] - m[(2,1)]*m[(1,2)]);
	let mut r2 = det*(m[(1,2)]*m[(2,0)] - m[(1,0)]*m[(2,2)]);
	let mut r3 = det*(m[(1,0)]*m[(2,1)] - m[(2,0)]*m[(1,1)]);

	let v1 = Vector3::new(r1,r2,r3);

	r1 = det*(m[(0,2)]*m[(2,1)] - m[(0,1)]*m[(2,2)]);
	r2 = det*(m[(0,0)]*m[(2,2)] - m[(0,2)]*m[(2,0)]);
	r3 = det*(m[(2,0)]*m[(0,1)] - m[(0,0)]*m[(2,1)]);

	let v2 = Vector3::new(r1,r2,r3);

	r1 = det*(m[(0,1)]*m[(1,2)] - m[(0,2)]*m[(1,1)]);
	r2 = det*(m[(1,0)]*m[(0,2)] - m[(0,0)]*m[(1,2)]);
	r3 = det*(m[(0,0)]*m[(1,1)] - m[(1,0)]*m[(0,1)]);

	let v3 = Vector3::new(r1,r2,r3);

	Matrix3::from_columns(&[v1,v2,v3])
}