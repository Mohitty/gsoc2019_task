extern crate nalgebra;

use nalgebra::*;

pub type PlaneCenter = Vector3<f32>;
pub type PlaneLocalAxis = Vector3<f32>;
pub type LocalToGlobal = Matrix3<f32>;
pub type GlobalPoint = Vector3<f32>;
pub type LocalPoint = Vector3<f32>;
pub type DimesionU = f32;
pub type DimesionV = f32;

//This can be implemented for generic plane shapes.
pub trait BoundCheck {
	fn inside_bounds(&self, point: LocalPoint) -> bool;
}

pub struct RectangularBounds {
	pub width: DimesionU,
	pub height: DimesionV
}

impl BoundCheck for RectangularBounds {
	fn inside_bounds(&self, point: LocalPoint) -> bool {
		if Real::abs(point.x) <= (self.width/2.0) && 
		   Real::abs(point.y) <= (self.height/2.0) {
			return true;
		} else {		
			return false;
		}
	}
}

//Definition of Plane
//Fields: (1) Center of the Plane.
//		  (2) Transformation Matrix for switching coordinate systems.
//		  (3) Bounds of the Plane.
pub struct Plane {
	pub plane_center: PlaneCenter,
	pub rot_matrix: LocalToGlobal,
	pub bounds: RectangularBounds
}

impl Plane {
	//Constructs a new Plane.
	//Arguments: (1) PlaneCenter
	//			 (2) Plane Local X axis
	//			 (3) Plane Local Y axis
	//			 (4) Plane Bounds
	pub fn new(center: PlaneCenter,
			diru: PlaneLocalAxis,
			dirv: PlaneLocalAxis,
			bounds: RectangularBounds) -> Plane {

		let mut dirz = *(&diru.cross(&dirv));
		dirz.normalize_mut();
				
		Plane {
			plane_center: center,
			rot_matrix: Matrix3::from_columns(&[diru, dirv,dirz]),
			bounds: bounds,
		}
	}

	//Converts a Point in Plane's local frame to global coordinate frame.
	pub fn local_to_global(&self, point: LocalPoint) -> Vector3<f32> {
		self.plane_center + self.rot_matrix*point
	}

	//Converts a Point in Global frame to Plane's Local coordinate frame.
	pub fn global_to_local(&self, point: GlobalPoint) -> Vector3<f32> {
		let inv = &self.rot_matrix.try_inverse().unwrap();
		inv*(point-self.plane_center)
	}

	//Checks if the given Point(Global coordinates) lies on the plane or not.
	//For local points it can be checked directly by calling inside_bounds().
	pub fn is_inside(&self, point: GlobalPoint) -> bool {
		let local_point = self.global_to_local(point);
		if local_point.z == 0.0 {
			return self.bounds.inside_bounds(local_point);
		} else {		
			return false;
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_local_to_global() {
		let bounds = RectangularBounds{width: 5.0, height: 5.0};
		let plane =  Plane::new(Vector3::new(0.0,0.0,1.0),			//Plane center
								Vector3::new(1.0, 0.0, 0.0),		//Local X axis
								Vector3::new(0.0,1.0,0.0),			//Local Y axis
								bounds);
		let local_pt = Vector3::new(1.0,1.0,0.0);
		let global_point = plane.local_to_global(local_pt);
		assert_eq!(global_point, Vector3::new(1.0,1.0,1.0));
	}

	#[test]
	fn test_global_to_local() {
		let bounds = RectangularBounds{width: 5.0, height: 5.0};
		let plane =  Plane::new(Vector3::new(0.0,0.0,1.0),			//Plane center
								Vector3::new(1.0, 0.0, 0.0),		//Local X axis
								Vector3::new(0.0,1.0,0.0),			//Local Y axis
								bounds);
		let global_pt = Vector3::new(1.0,1.0,1.0);
		let local_point = plane.global_to_local(global_pt);
		assert_eq!(local_point, Vector3::new(1.0,1.0,0.0));
	}

	#[test]
	fn test_is_inside() {
		let bounds = RectangularBounds{width: 5.0, height: 5.0};
		let plane =  Plane::new(Vector3::new(0.0,0.0,1.0),			//Plane center
								Vector3::new(1.0, 0.0, 0.0),		//Local X axis
								Vector3::new(0.0,1.0,0.0),			//Local Y axis
								bounds);
		
		let mut global_pt = Vector3::new(1.0,1.0,1.0);
		assert_eq!(plane.is_inside(global_pt), true);
		
		global_pt = Vector3::new(1.0,1.0,2.0);
		assert_eq!(plane.is_inside(global_pt), false);

		global_pt = Vector3::new(-10.0,1.0,1.0);
		assert_eq!(plane.is_inside(global_pt), false);
	}
}