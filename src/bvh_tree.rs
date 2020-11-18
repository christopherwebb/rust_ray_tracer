use rand;
use rand::Rng;

use crate::aabb::AABB;
use crate::ray::Ray;
use crate::primative::Primative;
use crate::core::Point3f;
use crate::shapes::base::{Interaction, ShapeTrait};


pub struct BVHTree {
    primatives: Vec<Primative>,
    nodes: Vec<Node>,
    root: Option<usize>,
}


impl BVHTree {
    pub fn blank() -> Self {
        return BVHTree {
            primatives: vec![],
            nodes: vec![],
            root: None,
        }
    }

    pub fn new(primatives: Vec<Primative>, time_0: f32, time_1: f32) -> Self {
        let mut nodes:Vec<Node> = vec![];
        let mut rnd_gen = rand::thread_rng();

        let mut prim_list: Vec<(usize, AABB)> = primatives
            .iter()
            .enumerate()
            .map(|(index, primative)| {
                (index, primative.bounding_box(time_0, time_1))
            })
            .filter(|(index, aabb_option)| aabb_option.is_some())
            .map(|(index, aabb_option)| {
                (index, aabb_option.unwrap())
            })
            .collect();
        

        if prim_list.is_empty() {
            BVHTree::blank()
        } else {
            let root = BVHTree::new_branch(
                &mut rnd_gen,
                &mut nodes,
                &mut prim_list,
            );

            BVHTree {
                nodes: vec![],
                root: Some(root),
                primatives: primatives,
            }}
    }

    fn new_branch(rnd_gen: &mut rand::rngs::ThreadRng, nodes: &mut Vec<Node>, primative_subsection: &mut Vec<(usize, AABB)>) -> usize {
        if primative_subsection.len() == 1 {
            let (primative, aabb) = primative_subsection[0];
            let node = Node::Leaf {
                primative: primative,
                aabb: aabb,
            };
            nodes.push(node);
            nodes.len() - 1
        } else {
            match rnd_gen.gen_range(0, 3) {
                0 => primative_subsection.sort_by(|(_, aabb_a), (_, aabb_b)| aabb_a.minimum.x.partial_cmp(&aabb_b.minimum.x).unwrap()),
                1 => primative_subsection.sort_by(|(_, aabb_a), (_, aabb_b)| aabb_a.minimum.y.partial_cmp(&aabb_b.minimum.y).unwrap()),
                _ => primative_subsection.sort_by(|(_, aabb_a), (_, aabb_b)| aabb_a.minimum.z.partial_cmp(&aabb_b.minimum.z).unwrap()),
            }
            
            let chunk_size = primative_subsection.len() / 2 + (primative_subsection.len() & 1);
            let mut vec_chunks_iter = primative_subsection.chunks(chunk_size);

            let mut left_chunk = vec_chunks_iter.next().unwrap().to_vec();
            let left_index = BVHTree::new_branch(rnd_gen, nodes, &mut left_chunk);

            let mut right_chunk = vec_chunks_iter.next().unwrap().to_vec();
            let right_index = BVHTree::new_branch(rnd_gen, nodes, &mut right_chunk);

            let node = Node::Branch {
                left: Some(left_index),
                right: Some(right_index),
                aabb: AABB::join(
                    &nodes[left_index].get_aabb(),
                    &nodes[right_index].get_aabb(),
                ),
            };

            nodes.push(node);
            nodes.len() - 1
        }

    }
    
    pub fn get_collision(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Interaction> {
        match self.root {
            Some(root_n) => self.get_collision_on(root_n, ray, t_min, t_max),
            None => None,
        }
    }

    fn get_collision_on(&self, index: usize, ray: &Ray, t_min: f32, t_max: f32) -> Option<Interaction> {
        let current_node = self.nodes[index];
        match current_node {
            Node::Branch {left, right, aabb} => {
                match aabb.hit(ray, t_min, t_max) {
                    true => {
                        // get left collision
                        let left_collsion = match left {
                            Some(left_index) => self.get_collision_on(left_index, ray, t_min, t_max),
                            None => None,
                        };

                        // get right collision, return it or the left collision as appropriate
                        match (left_collsion, right) {
                            (Some(left_col), Some(right_index)) => {
                                let right_col = self.get_collision_on(right_index, ray, t_min, left_col.t);
                                match right_col {
                                    Some(_) => right_col,
                                    None => left_collsion,
                                }
                            },
                            (          None, Some(right_index)) => self.get_collision_on(right_index, ray, t_min, t_max),
                            (Some(left_col),              None) => left_collsion,
                            (          None,              None) => None,
                        }
                    },
                    false => None,
                }
            },
            Node::Leaf { primative, aabb } => {
                match aabb.hit(ray, t_min, t_max) {
                    true  => {
                        let prima_clone = self.primatives[primative].clone();
                        prima_clone.collide(ray, t_min, t_max)
                    },
                    false => None,
                }
            },
        }
    }
}


#[derive(Copy, Clone)]
enum Node {
    Branch {
        left: Option<usize>,
        right: Option<usize>,
        aabb: AABB,
    },
    Leaf {
        primative: usize,
        aabb: AABB,
    },
}


impl Node {
    fn get_aabb(&self) -> AABB {
        match self {
            Self::Branch { left, right, aabb} => *aabb,
            Self::Leaf { primative, aabb } => *aabb,
        }
    }
}
