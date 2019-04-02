extern crate nalgebra;

use nalgebra::*;

pub type AffineTransformation = Transform3<f32>;
pub type GlobalPoint = Point3<f32>;
pub type LocalPoint = Point3<f32>;
pub type BoundDimesion = f32;

//This can be implemented for generic plane shapes.
pub trait BoundCheck {
	fn inside_bounds(&self, point: LocalPoint) -> bool;
}

pub struct RectangularBounds {
	pub width: BoundDimesion,
	pub height: BoundDimesion
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
//Fields: (1) Transformation matrix for changing from local to global
//		  (2) Transformation Matrix ffor changing from global to local.
//		  (3) Bounds of the Plane.
pub struct Plane {
	pub transformation: AffineTransformation,
	pub inv_transformation: AffineTransformation,
	pub bounds: RectangularBounds
}

impl Plane {
	//Constructs a new Plane.
	//Arguments: (1) Affine Transform
	//	     (2) Plane Bounds
	pub fn new(transform: AffineTransformation,
			bounds: RectangularBounds) -> Plane {

		let mut inv_t = transform;
		inv_t.try_inverse_mut();

		Plane {
			transformation: transform,
			inv_transformation: inv_t,
			bounds: bounds,
		}
	}

	//Converts a Point in Plane's local frame to global coordinate frame.
	pub fn local_to_global(&self, point: LocalPoint) -> Point3<f32> {
		self.transformation*point
	}

	//Converts a Point in Global frame to Plane's Local coordinate frame.
	pub fn global_to_local(&self, point: GlobalPoint) -> Point3<f32> {
		self.inv_transformation*point
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

	    let m = Matrix4::new(1.0,0.0,0.0,0.0,
	                         0.0,1.0,0.0,0.0,
	                         0.0,0.0,1.0,1.0,
	                         0.0,0.0,0.0,1.0);
	    let t :Transform3<f32> = Transform3::from_matrix_unchecked(m);

	    let plane = Plane::new(t,bounds);
		let local_pt = Point3::new(1.0,1.0,0.0);
		
		let global_point = plane.local_to_global(local_pt);
		assert_eq!(global_point, Point3::new(1.0,1.0,1.0));
	}

	#[test]
	fn test_global_to_local() {
	    let bounds = RectangularBounds{width: 5.0, height: 5.0};

	    let m = Matrix4::new(1.0,0.0,0.0,0.0,
	                         0.0,1.0,0.0,0.0,
	                         0.0,0.0,1.0,1.0,
	                         0.0,0.0,0.0,1.0);
	    let t :Transform3<f32> = Transform3::from_matrix_unchecked(m);

	    let plane = Plane::new(t,bounds);
		let global_pt = Point3::new(1.0,1.0,1.0);

		let local_point = plane.global_to_local(global_pt);
		assert_eq!(local_point, Point3::new(1.0,1.0,0.0));
	}

	#[test]
	fn test_is_inside() {
	    let bounds = RectangularBounds{width: 5.0, height: 5.0};

	    let m = Matrix4::new(1.0,0.0,0.0,0.0,
	                         0.0,1.0,0.0,0.0,
	                         0.0,0.0,1.0,1.0,
	                         0.0,0.0,0.0,1.0);
	    let t :Transform3<f32> = Transform3::from_matrix_unchecked(m);

	    let plane = Plane::new(t,bounds);
		
		let mut global_pt = Point3::new(1.0,1.0,1.0);
		assert_eq!(plane.is_inside(global_pt), true);
		
		global_pt = Point3::new(1.0,1.0,2.0);
		assert_eq!(plane.is_inside(global_pt), false);

		global_pt = Point3::new(-10.0,1.0,1.0);
		assert_eq!(plane.is_inside(global_pt), false);
	}
}
