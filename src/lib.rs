use futures::future::join_all;

const THRESHOLD: usize = 10;

async fn map_chunk<T, U, F>(chunk: Vec<T>, func: &F) -> Vec<U>
where
    F: Fn(T) -> U {
    let mut result = Vec::with_capacity(THRESHOLD);

    for element in chunk {
        result.push(func(element));
    }

    result
}

pub async fn parallel_map<T, U, F>(mut array: Vec<T>, func: F) -> Vec<U>
where
    U: Clone,
    F: Fn(T) -> U {
    let mut futures_vec = Vec::with_capacity(array.len());
    
    futures_vec.push(map_chunk(array.split_off(array.len() % THRESHOLD), &func));
    while array.len() > 0 {
        futures_vec.push(map_chunk(array.split_off(THRESHOLD), &func));
    }

    join_all(futures_vec).await.concat()
}


#[cfg(test)]
mod tests {
    use super::parallel_map;

    const RANDOM_VECTOR_LEN: usize = 10_000;

    fn generate_vector(len: usize, seed: u64) -> Vec<i32> {
        use rand::prelude::*;
        use rand_pcg::Pcg64;

        let mut generator = Pcg64::seed_from_u64(seed);
        let mut result = vec![];

        for _ in 0..len {
            result.push(generator.gen::<i32>());
        }

        result
    }

    async fn same_length(vec: Vec<i32>) {
        let func = |x: i32| x + 42;

        let my_res = parallel_map(vec.clone(), func).await;
        let ok_res = vec.into_iter().map(func).collect::<Vec<_>>();

        assert_eq!(my_res.len(), ok_res.len());
    }

    async fn simple_lambda(vec: Vec<i32>) {
        let func = |x: i32| x + 42;

        let my_res = parallel_map(vec.clone(), func).await;
        let ok_res = vec.into_iter().map(func).collect::<Vec<_>>();

        assert_eq!(my_res, ok_res);
    }

    async fn composition(vec: Vec<i32>) {
        let add1 = |x: i32| x + 1;
        let add2 = |x: i32| x + 2;
        let composition = |x| add2(add1(x));


        let ok_res = vec.clone().into_iter().map(composition).collect::<Vec<_>>();

        let my_res = parallel_map(
            parallel_map(vec.clone(), add1).await,
            add2,
        ).await;

        assert_eq!(my_res, ok_res);

    
        let my_composition_res = parallel_map(vec.clone(), composition).await;

        assert_eq!(my_composition_res, ok_res);
    }

    #[tokio::test]
    async fn test_empty_vector() {
        let vec = vec![];

        let func = |x: i32| x + 42;

        let my_res = parallel_map(vec.clone(), func).await;
        let ok_res = vec.into_iter().map(func).collect::<Vec<_>>();

        assert_eq!(my_res, ok_res);
    }

    #[tokio::test]
    async fn test_panic_lambda_empty_vector() {
        let vec = vec![];

        let func = |_x: i32| panic!();
        
        parallel_map(vec.clone(), func).await;
    }

    #[tokio::test]
    async fn test_same_length() {
        same_length(generate_vector(RANDOM_VECTOR_LEN, 42)).await;
    }

    #[tokio::test]
    async fn test_simple_lambda() {
        simple_lambda(generate_vector(RANDOM_VECTOR_LEN, 42)).await;
    }

    #[tokio::test]
    async fn test_composition() {
        composition(generate_vector(RANDOM_VECTOR_LEN, 42)).await;
    }
}
