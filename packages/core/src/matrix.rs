/**
 * 矩阵，帮助棋盘操作数据
 */
pub struct Matrix {}

impl Matrix {
    /**
     * 逆时针旋转
     */
    pub fn rotate_left(matrix: &mut [[u32; 4]; 4]) {
        *matrix = [
            [matrix[0][3], matrix[1][3], matrix[2][3], matrix[3][3]],
            [matrix[0][2], matrix[1][2], matrix[2][2], matrix[3][2]],
            [matrix[0][1], matrix[1][1], matrix[2][1], matrix[3][1]],
            [matrix[0][0], matrix[1][0], matrix[2][0], matrix[3][0]],
        ];
    }
    /**
     * 顺时针旋转
     */
    pub fn rotate_right(matrix: &mut [[u32; 4]; 4]) {
        *matrix = [
            [matrix[3][0], matrix[2][0], matrix[1][0], matrix[0][0]],
            [matrix[3][1], matrix[2][1], matrix[1][1], matrix[0][1]],
            [matrix[3][2], matrix[2][2], matrix[1][2], matrix[0][2]],
            [matrix[3][3], matrix[2][3], matrix[1][3], matrix[0][3]],
        ];
    }
    /**
     * 上下倒转(镜像)
     */
    pub fn reverse(matrix: &mut [[u32; 4]; 4]) {
        *matrix = [
            [matrix[3][0], matrix[3][1], matrix[3][2], matrix[3][3]],
            [matrix[2][0], matrix[2][1], matrix[2][2], matrix[2][3]],
            [matrix[1][0], matrix[1][1], matrix[1][2], matrix[1][3]],
            [matrix[0][0], matrix[0][1], matrix[0][2], matrix[0][3]],
        ];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn rotate_left() {
        let mut matrix = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        Matrix::rotate_left(&mut matrix);
        assert_eq!(
            matrix,
            [
                [4, 8, 12, 16],
                [3, 7, 11, 15],
                [2, 6, 10, 14],
                [1, 5, 9, 13],
            ]
        );
    }

    #[test]
    fn rotate_right() {
        let mut matrix = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        Matrix::rotate_right(&mut matrix);
        assert_eq!(
            matrix,
            [
                [13, 9, 5, 1,],
                [14, 10, 6, 2,],
                [15, 11, 7, 3,],
                [16, 12, 8, 4,],
            ]
        );
    }

    #[test]
    fn reverse() {
        let mut matrix = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        Matrix::reverse(&mut matrix);
        assert_eq!(
            matrix,
            [
                [13, 14, 15, 16],
                [9, 10, 11, 12],
                [5, 6, 7, 8],
                [1, 2, 3, 4],
            ]
        );
    }
}
