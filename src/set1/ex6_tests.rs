#[cfg(test)]
mod transpose_blocks_tests {
    use crate::set1::ex6::transpose_blocks;

    #[test]
    fn test_transpose_blocks_empty_input() {
        let blocks: Vec<Vec<u8>> = vec![];
        let keylength: usize = 4;
        let result = transpose_blocks(&blocks, keylength);

        let expected: Vec<Vec<u8>> = vec![vec![]; keylength as usize];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_transpose_blocks_single_block() {
        let blocks: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4]];
        let keylength: usize = 4;
        let result = transpose_blocks(&blocks, keylength);

        let expected: Vec<Vec<u8>> = vec![vec![1], vec![2], vec![3], vec![4]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_transpose_blocks_multiple_blocks() {
        let blocks: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let keylength: usize = 4;
        let result = transpose_blocks(&blocks, keylength);

        let expected: Vec<Vec<u8>> = vec![
            vec![1, 5, 9],
            vec![2, 6, 10],
            vec![3, 7, 11],
            vec![4, 8, 12],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_transpose_blocks_uneven_blocks() {
        let blocks: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7], vec![9, 10, 11, 12]];
        let keylength: usize = 4;
        let result = transpose_blocks(&blocks, keylength);

        let expected: Vec<Vec<u8>> =
            vec![vec![1, 5, 9], vec![2, 6, 10], vec![3, 7, 11], vec![4, 12]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_transpose_blocks_varying_keylength() {
        let blocks: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let keylength: usize = 2;
        let result = transpose_blocks(&blocks, keylength);

        let expected: Vec<Vec<u8>> = vec![vec![1, 5, 9], vec![2, 6, 10]];
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod split_blocks_tests {
    use crate::set1::ex6::split_c_blocks;

    #[test]
    fn test_split_c_blocks_empty_input() {
        let cyphertext_bytes: Vec<u8> = vec![];
        let keylength: usize = 4;
        let result = split_c_blocks(&cyphertext_bytes, keylength);

        let expected: Vec<Vec<u8>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_c_blocks_single_block() {
        let cyphertext_bytes: Vec<u8> = vec![1, 2, 3, 4];
        let keylength: usize = 4;
        let result = split_c_blocks(&cyphertext_bytes, keylength);

        let expected: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_c_blocks_multiple_blocks() {
        let cyphertext_bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let keylength: usize = 4;
        let result = split_c_blocks(&cyphertext_bytes, keylength);

        let expected: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_c_blocks_uneven_input() {
        let cyphertext_bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
        let keylength: usize = 4;
        let result = split_c_blocks(&cyphertext_bytes, keylength);

        let expected: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_c_blocks_varying_keylength() {
        let cyphertext_bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let keylength: usize = 2;
        let result = split_c_blocks(&cyphertext_bytes, keylength);

        let expected: Vec<Vec<u8>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(result, expected);
    }
}
