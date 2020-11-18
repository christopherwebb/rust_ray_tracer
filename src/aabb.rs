use crate::core::{
    Normal3f,
    Point3f,
    Matrix4x4f,
    dot_vv,
    dot_vn,
};
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct AABB {
    pub minimum: Point3f,
    pub maximum: Point3f,
}

impl AABB {
    pub fn hit(self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let inv_dir = 1.0_f32 / r.direction().x;
        let t0 = (self.minimum.x - r.origin().x) * inv_dir;
        let t1 = (self.maximum.x - r.origin().x) * inv_dir;

        let (t0, t1) = if inv_dir < 0.0_f32 {
            (t1, t0)
        } else {
            (t0, t1)
        };

        let t_min = if t0 > t_min { t0 } else { t_min };
        let t_min = if t1 > t_max { t1 } else { t_max };

        if t_max <= t_min {
            return false;
        }

        let inv_dir = 1.0_f32 / r.direction().y;
        let t0 = (self.minimum.y - r.origin().y) * inv_dir;
        let t1 = (self.maximum.y - r.origin().y) * inv_dir;

        let (t0, t1) = if inv_dir < 0.0_f32 {
            (t1, t0)
        } else {
            (t0, t1)
        };

        let t_min = if t0 > t_min { t0 } else { t_min };
        let t_min = if t1 > t_max { t1 } else { t_max };

        if t_max <= t_min {
            return false;
        }

        let inv_dir = 1.0_f32 / r.direction().z;
        let t0 = (self.minimum.z - r.origin().z) * inv_dir;
        let t1 = (self.maximum.z - r.origin().z) * inv_dir;

        let (t0, t1) = if inv_dir < 0.0_f32 {
            (t1, t0)
        } else {
            (t0, t1)
        };

        let t_min = if t0 > t_min { t0 } else { t_min };
        let t_min = if t1 > t_max { t1 } else { t_max };

        if t_max <= t_min {
            return false;
        }

        true

        // for (int a = 0; a < 3; a++) {
        //     auto invD = 1.0f / r.direction()[a];
        //     auto t0 = (min()[a] - r.origin()[a]) * invD;
        //     auto t1 = (max()[a] - r.origin()[a]) * invD;
        //     if (invD < 0.0f)
        //         std::swap(t0, t1);
        //     t_min = t0 > t_min ? t0 : t_min;
        //     t_max = t1 < t_max ? t1 : t_max;
        //     if (t_max <= t_min)
        //         return false;
        // }
        // return true;
    }

    pub fn join(box_a: &AABB, box_b: &AABB) -> AABB {
        AABB {
            minimum: Point3f {
                x: box_a.minimum.x.min(box_b.minimum.x),
                y: box_a.minimum.y.min(box_b.minimum.y),
                z: box_a.minimum.z.min(box_b.minimum.z),
            },
            maximum: Point3f {
                x: box_a.maximum.x.max(box_b.maximum.x),
                y: box_a.maximum.y.max(box_b.maximum.y),
                z: box_a.maximum.z.max(box_b.maximum.z),
            },
        }
    }
}