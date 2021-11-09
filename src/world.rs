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

    pub fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::new();
        for object in &self.objects {
            object.intersect(&r, &mut intersections);
        }
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }

    pub fn shade_hit(&self, comps: &PreparedComputations) -> Color {
        let mut lights = self.lights.iter();
        match lights.next() {
            Some(light) => {
                let in_shadow = self.in_shadow(light, &comps.over_point);
                let mut color = comps.object.render_at(comps, light, in_shadow);
                for light in lights {
                    let in_shadow = self.in_shadow(light, &comps.over_point);
                    color = color + comps.object.render_at(comps, light, in_shadow);
                }
                color
            }
            None => BLACK,
        }
    }

    pub fn color_at(&self, r: &Ray) -> Color {
        let intersections = self.intersect(r);
        let hit = hit(intersections);
        let color = match hit {
            Some(h) => {
                let comps = h.prepare_computations(&r);
                self.shade_hit(&comps)
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

    pub fn in_shadow(&self, light: &PointLight, point: &Point) -> bool {
        let v = light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalized();

        let r = Ray::new(*point, direction);
        let intersections = self.intersect(&r);

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
        let xs = w.intersect(&r);
        assert_eq!(xs.len(), 4);
        assert!(epsilon_equal(xs[0].t, 4.));
        assert!(epsilon_equal(xs[1].t, 4.5));
        assert!(epsilon_equal(xs[2].t, 5.5));
        assert!(epsilon_equal(xs[3].t, 6.));
    }

    #[test]
    fn shade_intersection() {
        let w = World::test_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = w.objects.first().unwrap();
        let s = &**shape;
        let i = Intersection::new(4.0, s);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
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
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn ray_misses() {
        let w = World::test_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 1, 0));
        let c = w.color_at(&r);
        assert_eq!(c, BLACK);
    }

    #[test]
    fn ray_hits() {
        let w = World::test_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let c = w.color_at(&r);
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
        let c = w.color_at(&r);
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
        let shadowed = {
            let light = w.lights()[0];
            w.in_shadow(&light, &p)
        };
        assert_eq!(shadowed, false);
    }

    #[test]
    fn shadow_object_between_point_and_light() {
        let w = World::test_world();
        let p = Point::new(10, -10, 10);
        let shadowed = {
            let light = w.lights()[0];
            w.in_shadow(&light, &p)
        };
        assert_eq!(shadowed, true);
    }

    #[test]
    fn shadow_object_behind_light() {
        let w = World::test_world();
        let p = Point::new(-20, 20, -20);
        let shadowed = {
            let light = w.lights()[0];
            w.in_shadow(&light, &p)
        };
        assert_eq!(shadowed, false);
    }

    #[test]
    fn shadow_object_behind_point() {
        let w = World::test_world();
        let p = Point::new(-2, 2, -2);
        let shadowed = {
            let light = w.lights()[0];
            w.in_shadow(&light, &p)
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

        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
