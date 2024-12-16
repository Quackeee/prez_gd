use glm::{Mat4, Quat, Vec3, Vec4};


pub struct DecomposedMatrix {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl DecomposedMatrix {
    fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }
}

pub struct MatrixUtils;

impl MatrixUtils {
    pub fn get_matrix_column(matrix: Mat4, column: usize) -> Vec4 {
        let col_index = 4 * column;
        Vec4::new(matrix[col_index], matrix[col_index + 1], matrix[col_index + 2], matrix[col_index + 3])
    }

    pub fn get_translation(matrix: Mat4) -> Vec3 {
        MatrixUtils::get_matrix_column(matrix, 3).xyz()
    }

    pub fn get_scale(matrix: Mat4) -> Vec3 {
        let c0 = MatrixUtils::get_matrix_column(matrix, 0);
        let c1 = MatrixUtils::get_matrix_column(matrix, 1);
        let c2 = MatrixUtils::get_matrix_column(matrix, 2);

        Vec3::new(c0.magnitude(), c1.magnitude(), c2.magnitude())
    }

    pub fn get_rotation_with_scale(matrix: Mat4, scale: Vec3) -> Quat {
        let mut c0: Vec4 = MatrixUtils::get_matrix_column(matrix, 0) / scale.x;
        c0.w = 0.0;

        let mut c1 = MatrixUtils::get_matrix_column(matrix, 1) / scale.y;
        c1.w = 0.0;

        let mut c2 = MatrixUtils::get_matrix_column(matrix, 2) / scale.z;
        c2.w = 0.0;

        glm::mat3_to_quat(&glm::Mat3::from_columns(&[c0.xyz(), c1.xyz(), c2.xyz()]))
    }

    pub fn get_rotation(matrix: Mat4) -> Quat {
        MatrixUtils::get_rotation_with_scale(matrix, Vec3::new(1.0, 1.0, 1.0))
    }

    pub fn decompose_matrix(matrix: Mat4) -> DecomposedMatrix {
        let translation = MatrixUtils::get_translation(matrix);
        let rotation = MatrixUtils::get_rotation(matrix);
        let scale = MatrixUtils::get_scale(matrix);


        DecomposedMatrix::new(translation, rotation, scale)
    }
}