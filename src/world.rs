use crate::{
    color::{Color, BLACK},
    intersection::{hit, Intersection, PreparedComputations},
    light::PointLight,
    material::{Material, Shininess},
    matrix::Mat4,
    ray::Ray,
    shapes::shape::Shape,
    shapes::sphere::Sphere,
    tuple::Point,
};

#[derive(Debug)]
pub struct World {
    objects: Vec<Box<dyn Shape>>,
    lights: Vec<PointLight>,
}

impl World {
    pub fn test_world() -> Self {
        let color_s1 = Color::new(0.8, 1.0, 0.6);

        let shininess: Shininess = 200_usize as Shininess;

        let material_s1 = Material::new(color_s1, 0.1, 0.7, 0.2, shininess);
        let mut s1 = Sphere::default();
        s1.material = material_s1;

        let transform_s2 = Mat4::new_scaling(0.5, 0.5, 0.5);
        let mut s2 = Sphere::default();
        s2.set_transformation(transform_s2);

        let objects: Vec<Box<dyn Shape>> = vec![Box::new(s1), Box::new(s2)];

        let lights = vec![PointLight::new(
            Point::new(-10, 10, -10),
            Color::new(1.0, 1.0, 1.0),
        )];

        Self { objects, lights }
    }

    /// Tries to intersect the ray with all objects in the world.
    /// Results are written to the provided "intersections" vector, which can be re-used later to save on allocations.
    pub(crate) fn intersect<'a>(&'a self, r: &Ray, intersections: &mut Vec<Intersection<'a>>) {
        for object in &self.objects {
            object.intersect(r, intersections);
        }
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }

    /// Given the prepared computations of the point a ray hit, this function determines the color at this point by first determining the lighting conditions and then rendering the point by accessing its material's render method.
    /// The intersections vector is only provided to save on allocations. If you did not get it, just pass an empty vector.
    pub(crate) fn shade_hit<'a>(
        &'a self,
        comps: &PreparedComputations,
        intersections: &mut Vec<Intersection<'a>>,
    ) -> Color {
        let mut lights = self.lights.iter();
        match lights.next() {
            Some(light) => {
                let in_shadow = self.in_shadow(light, &comps.over_point, intersections);
                let mut color = comps.object.render_at(comps, light, in_shadow);
                for light in lights {
                    let in_shadow = self.in_shadow(light, &comps.over_point, intersections);
                    color = color + comps.object.render_at(comps, light, in_shadow);
                }
                color
            }
            None => BLACK,
        }
    }

    /// Determines th color a ray produces.
    /// If it does not hit, returns BLACK.
    /// If it hits, returns the result of the rendered point.
    /// The intersections argument is only for saving on allocations - if in doubt, pass a new vector.
    pub(crate) fn color_at<'a>(
        &'a self,
        r: &Ray,
        intersections: &mut Vec<Intersection<'a>>,
    ) -> Color {
        self.intersect(r, intersections);
        let hit = hit(intersections);
        let color = match hit {
            Some(h) => {
                let comps = h.prepare_computations(r);
                self.shade_hit(&comps, intersections)
            }
            None => BLACK,
        };
        color
    }

    pub fn add_object(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }
    /// Moves objects out of the given vector into the scene
    pub fn add_objects(&mut self, objects: &mut Vec<Box<dyn Shape>>) {
        self.objects.append(objects);
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }
    /// Moves lights out of the given vector into the scene
    pub fn add_lights(&mut self, lights: &mut Vec<PointLight>) {
        self.lights.append(lights);
    }

    pub fn objects(&self) -> &Vec<Box<dyn Shape>> {
        &self.objects
    }

    pub fn lights(&self) -> &Vec<PointLight> {
        &self.lights
    }

    pub(crate) fn in_shadow<'a>(
        &'a self,
        light: &PointLight,
        point: &Point,
        intersections: &mut Vec<Intersection<'a>>,
    ) -> bool {
        let v = light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalized();

        let r = Ray::new(*point, direction);
        self.intersect(&r, intersections);

        let h = hit(intersections);

        match h {
            Some(intersection) => intersection.t < distance,
            None => false,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            objects: Default::default(),
            lights: Default::default(),
        }
    }
}

#[cfg(test)]
mod world_tests {
    use crate::{
        color::{Color, BLACK, WHITE},
        epsilon::epsilon_equal,
        intersection::Intersection,
        light::PointLight,
        material::Material,
        matrix::Mat4,
        ray::Ray,
        shapes::sphere::Sphere,
        tuple::{Point, Vector},
        world::World,
    };

    #[test]
    fn new() {
        let world = World::default();
        assert_eq!(world.objects.len(), 0);
        assert_eq!(world.lights.len(), 0);
    }

    #[test]
    fn new_test_default() {
        let w = World::test_world();

        let light = PointLight::new(Point::new(-10, 10, -10), Color::new(1, 1, 1));
        let mut s = Sphere::default();
        let mut mat = Material::default();
        mat.color = Color::new(0.8, 1.0, 0.6);
        mat.diffuse = 0.7;
        mat.specular = 0.2;
        s.material = mat;
        let mut s2 = Sphere::default();
        let transf = Mat4::new_scaling(0.5, 0.5, 0.5);
        s2.set_transformation(transf);

        assert_eq!(w.lights, vec!(light));
        let ws1 = w.objects[0].as_any().downcast_ref::<Sphere>().unwrap();
        let ws2 = w.objects[1].as_any().downcast_ref::<Sphere>().unwrap();
        assert_eq!(ws1, &s);
        assert_eq!(ws2, &s2);
    }

    #[test]
    fn intersect_with_ray() {
        let w = World::test_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0, 0, 1));
        let mut intersections = Vec::new();
        w.intersect(&r, &mut intersections);
        assert_eq!(intersections.len(), 4);
        assert!(epsilon_equal(intersections[0].t, 4.));
        assert!(epsilon_equal(intersections[1].t, 4.5));
        assert!(epsilon_equal(intersections[2].t, 5.5));
        assert!(epsilon_equal(intersections[3].t, 6.));
    }

    #[test]
    fn shade_intersection() {
        let w = World::test_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = w.objects.first().unwrap();
        let s = &**shape;
        let i = Intersection::new(4.0, s);
        let comps = i.prepare_computations(&r);
        let mut intersections = Vec::new();
        let c = w.shade_hit(&comps, &mut intersections);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shade_intersection_inside() {
        let mut w = World::test_world();
        w.lights = vec![PointLight::new(
            Point::new(0.0, 0.25, 0.0),
            Color::new(1, 1, 1),
        )];
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let s = &*w.objects[1];

        let i = Intersection::new(0.5, s);
        let mut intersections = Vec::new();
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps, &mut intersections);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn ray_misses() {
        let w = World::test_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 1, 0));
        let mut intersections = Vec::new();
        let c = w.color_at(&r, &mut intersections);
        assert_eq!(c, BLACK);
    }

    #[test]
    fn ray_hits() {
        let w = World::test_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut intersections = Vec::new();
        let c = w.color_at(&r, &mut intersections);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn intersection_behind_ray() {
        let mut w = World::test_world();
        let material = w.objects[0].material_mut();
        material.ambient = 1.0;

        let material = w.objects[1].material_mut();
        material.ambient = 1.0;
        let inner_color = material.color;

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let mut intersections = Vec::new();
        let c = w.color_at(&r, &mut intersections);
        assert_eq!(c, inner_color);
    }

    #[test]
    fn add_object() {
        let mut w = World::default();
        let s = Box::new(Sphere::default());
        w.add_object(s);

        let first = w
            .objects
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<Sphere>()
            .unwrap();
        assert_eq!(w.objects.len(), 1);
        assert_eq!(first, &Sphere::default());
    }

    #[test]
    fn add_objects() {
        let mut w = World::default();
        assert_eq!(w.objects.len(), 0);
        let s = Box::new(Sphere::default());
        let s2 = Box::new(Sphere::default());

        w.add_objects(&mut vec![s, s2]);
        assert_eq!(w.objects.len(), 2);
    }

    #[test]
    fn add_light() {
        let mut w = World::default();
        assert_eq!(w.lights.len(), 0);

        let l = PointLight::new(Point::new(0, 0, 0), WHITE);
        w.add_light(l);
        assert_eq!(w.lights.len(), 1);
    }

    #[test]
    fn add_lights() {
        let mut w = World::default();
        assert_eq!(w.lights.len(), 0);

        let l = PointLight::new(Point::new(0, 0, 0), WHITE);
        let l2 = PointLight::new(Point::new(0, 0, 0), WHITE);

        w.add_lights(&mut vec![l, l2]);
        assert_eq!(w.lights.len(), 2);
    }

    #[test]
    fn no_shadow() {
        let w = World::test_world();
        let p = Point::new(0, 10, 0);
        let mut intersections = Vec::new();
        let shadowed = {
            let light = w.lights()[0];
            w.in_shadow(&light, &p, &mut intersections)
        };
        assert_eq!(shadowed, false);
    }

    #[test]
    fn shadow_object_between_point_and_light() {
        let w = World::test_world();
        let p = Point::new(10, -10, 10);
        let mut intersections = Vec::new();
        let shadowed = {
            let light = w.lights()[0];
            w.in_shadow(&light, &p, &mut intersections)
        };
        assert_eq!(shadowed, true);
    }

    #[test]
    fn shadow_object_behind_light() {
        let w = World::test_world();
        let p = Point::new(-20, 20, -20);
        let mut intersections = Vec::new();
        let shadowed = {
            let light = w.lights()[0];
            w.in_shadow(&light, &p, &mut intersections)
        };
        assert_eq!(shadowed, false);
    }

    #[test]
    fn shadow_object_behind_point() {
        let w = World::test_world();
        let p = Point::new(-2, 2, -2);
        let mut intersections = Vec::new();
        let shadowed = {
            let light = w.lights()[0];
            w.in_shadow(&light, &p, &mut intersections)
        };
        assert_eq!(shadowed, false);
    }

    #[test]
    fn shade_hit_shadowed() {
        let mut w = World::default();
        w.add_light(PointLight::new(Point::new(0, 0, -10), WHITE));

        let s1 = Sphere::default();
        w.add_object(Box::new(s1));

        let mut s2 = Sphere::default();
        s2.set_transformation(Mat4::new_translation(0, 0, 10));
        w.add_object(Box::new(s2));

        let s2 = &*w.objects[1];

        let r = Ray::new(Point::new(0, 0, 5), Vector::new(0, 0, 1));
        let i = Intersection::new(4, s2);

        let comps = i.prepare_computations(&r);
        let mut intersections = Vec::new();
        let c = w.shade_hit(&comps, &mut intersections);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
