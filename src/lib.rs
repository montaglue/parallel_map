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

    while array.len() > THRESHOLD {
        futures_vec.push(map_chunk(array.split_off(THRESHOLD), &func));
    }
    futures_vec.push(map_chunk(array, &func));

    join_all(futures_vec).await.concat()
}
