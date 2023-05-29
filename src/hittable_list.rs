use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn default() -> HittableList {
        return HittableList { objects: vec![] };
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut record: Option<HitRecord> = None;

        for o in &self.objects {
            let result = o.hit(ray, t_min, closest_so_far);
            if let Some(v) = result {
                closest_so_far = v.t;
                record = Some(v);
            }
        }
        return record;
    }
}
