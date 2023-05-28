use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for o in &self.objects {
            let mut tmp_record: HitRecord = HitRecord::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                0.0,
                false,
            );
            if o.hit(ray, t_min, closest_so_far, &mut tmp_record) {
                hit_anything = true;
                closest_so_far = tmp_record.t;
                *record = tmp_record;
            }
        }
        return hit_anything;
    }
}
