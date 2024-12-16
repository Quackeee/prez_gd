use std::{cell::RefCell, rc::Weak};

use glm::{Mat4, Quat, Vec3, Vec4};

use crate::utils::matrix_utils::MatrixUtils;

pub struct TransformData {
    pub local_position: Vec3,
    pub local_rotation: Quat,
    pub local_scale: Vec3,

    pub global_position: Vec3,
    pub global_rotation: Quat,
    pub global_scale: Vec3,

    pub world_space_matrix: Mat4,
}

impl TransformData {
    pub fn new() -> Self {
        Self {
            local_position: Vec3::zeros(),
            local_rotation: Quat::identity(),
            local_scale: Vec3::new(1.0, 1.0, 1.0),

            global_position: Vec3::zeros(),
            global_rotation: Quat::identity(),
            global_scale: Vec3::new(1.0, 1.0, 1.0),

            world_space_matrix: Mat4::identity(),
        }
    }
}

// TODO: Testing, testing, and more testing
#[allow(dead_code)]
pub trait Transform {
    fn get_data_mut(&mut self) -> &mut TransformData;
    fn get_data(&self) -> &TransformData;
    fn get_parent(&self) -> Option<Weak<RefCell<Self>>>;
    fn get_children(&self) -> Vec<Weak<RefCell<Self>>>;

    fn calculate_local_model_matrix(&self) -> Mat4 {
        let mut translation_matrix = Mat4::identity();
        translation_matrix = glm::translate(&translation_matrix, &self.get_data().local_position);
        translation_matrix = translation_matrix * glm::quat_to_mat4(&self.get_data().local_rotation);
        translation_matrix = glm::scale(&translation_matrix, &self.get_data().local_scale);

        translation_matrix
    }

    fn update_matrix(&mut self) {
        let mut parent_matrix = Mat4::identity();
        let parent = self.get_parent();
        if parent.is_some() {
            let parent = parent.unwrap().upgrade().unwrap();
            parent_matrix = parent.borrow().get_data().world_space_matrix;
        }

        self.get_data_mut().world_space_matrix = parent_matrix * self.calculate_local_model_matrix();
        let dec = MatrixUtils::decompose_matrix(self.get_data().world_space_matrix);
        self.get_data_mut().global_position = dec.translation;
        self.get_data_mut().global_rotation = dec.rotation;
        self.get_data_mut().global_scale = dec.scale;

        // self.get_object().upgrade().unwrap().borrow_mut().update_children_transform_matrix();
        let children = self.get_children();
        for child in children {
            child.upgrade().unwrap().borrow_mut().update_matrix();
        }
    }

    fn update_matrix_with_locals(&mut self) {
        let decomp = MatrixUtils::decompose_matrix(self.calculate_local_model_matrix());
        self.get_data_mut().local_position = decomp.translation;
        self.get_data_mut().local_rotation = decomp.rotation;
        self.get_data_mut().local_scale = decomp.scale;

        self.update_matrix();
    }

    fn get_world_space_matrx(&self) -> Mat4 {
        self.get_data().world_space_matrix
    }

    fn set_global_position(&mut self, position: Vec3) {
        let parent = self.get_parent();
        if parent.is_some() {
            let parent_matrix = parent.unwrap().upgrade().unwrap().borrow().get_world_space_matrx();
            let parent_inverse = glm::inverse(&parent_matrix);
            let local_position = glm::Vec4::new(position.x, position.y, position.z, 1.0);

            let relative_position: Vec4 = parent_inverse * local_position;
            self.get_data_mut().local_position = relative_position.xyz();
        }
        else {
            self.get_data_mut().local_position = position;
        }

        self.update_matrix();
    }

    fn set_global_rotation(&mut self, rotation: Quat) {
        let parent = self.get_parent();
        if parent.is_some() {
            let parent_matrix = parent.unwrap().upgrade().unwrap().borrow().get_world_space_matrx();
            let parent_inverse = glm::inverse(&parent_matrix);
            let local_rotation = glm::quat_to_mat4(&self.get_data().local_rotation);
            let relative_rotation = parent_inverse * local_rotation;
            let decomp = MatrixUtils::decompose_matrix(relative_rotation);
            self.get_data_mut().local_rotation = decomp.rotation;
        }
        else {
            self.get_data_mut().local_rotation = rotation;
        }

        self.update_matrix();
    }

    fn set_local_position(&mut self, position: Vec3) {
        self.get_data_mut().local_position = position;

        self.update_matrix();
    }

    fn set_local_scale(&mut self, scale: Vec3) {
        self.get_data_mut().local_scale = scale;

        self.update_matrix();
    }

    fn set_local_rotation(&mut self, rotation: Quat) {
        self.get_data_mut().local_rotation = rotation;

        self.update_matrix();
    }

    fn get_local_position(&self) -> Vec3 {
        self.get_data().local_position
    }

    fn get_local_rotation(&self) -> Quat {
        self.get_data().local_rotation
    }

    fn get_local_scale(&self) -> Vec3 {
        self.get_data().local_scale
    }

    fn get_global_position(&self) -> Vec3 {
        self.get_data().global_position
    }

    fn get_global_rotation(&self) -> Quat {
        self.get_data().global_rotation
    }

    fn get_global_scale(&self) -> Vec3 {
        self.get_data().global_scale
    }

    fn get_forward_vector(&self) -> Vec3 {
        glm::quat_rotate_vec3(&self.get_data().global_rotation, &Vec3::new(0.0, 0.0, 1.0))
    }

    fn get_up_vector(&self) -> Vec3 {
        glm::quat_rotate_vec3(&self.get_data().global_rotation, &Vec3::new(0.0, 1.0, 0.0))
    }

    fn get_right_vector(&self) -> Vec3 {
        glm::quat_rotate_vec3(&self.get_data().global_rotation, &Vec3::new(1.0, 0.0, 0.0))
    }
}