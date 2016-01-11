// Extrait d'un code finalement non inclus

thread_local!(static PICROSS_ROWS_CACHE: RefCell<HashMap<(usize, Vec<usize>), Box<Vec<Vec<Cell>>>>> = RefCell::new(HashMap::new()));
/// Returns an iterator yielding all possible picross rows following the given constraints :
/// row_size: size of the row
/// spec: specification of the blocks : &vec![1,2] means a one-cell block and a two-cell block
fn gen_picross_rows(row_size: usize, spec: &Vec<usize>) -> Box<Vec<Vec<Cell>>> {
    PICROSS_ROWS_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let key = (row_size, spec.clone());
        {
            let cached: Option<&Box<Vec<Vec<Cell>>>> = cache.get(&key);
            if let Some(cached) = cached {
                return cached.clone()
            }
        }
        let new = Box::new(PicrossRowGenerator {
            row_size: row_size,
            spec: &spec,
            inc_series_gen: gen_increasing_series(spec.len(), row_size + 1 - spec.iter().fold(0, |sum, x| sum + x))
        }.collect::<Vec<Vec<Cell>>>());
        cache.insert(key, new.clone());
        new
    })
}
