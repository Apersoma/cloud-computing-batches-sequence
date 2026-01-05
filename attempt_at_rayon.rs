



// specifically this is testing it for phi_2
pub fn main() {
    let phi= 11;
    let offset = 0;
    let omicron = phi*phi;

    let mut sets = hashset(omicron as usize + phi as usize);

    let phi_n1 = phi-1;

    let indices_to_base_value = move |row: Int, column: Int| offset+row*phi_n1+column;
    
    // let mut iter0 = (0..phi+1)
    // #[cfg(target_pointer_width = "64")]
    let mut iter0 = (1..=phi)
        .map(|i|{
            let mut mini_set = BTreeSet::new();
            insert_unique_btree!(mini_set, offset);
            for ii in 1..phi {
                insert_unique_btree!(mini_set, indices_to_base_value(i,ii));
            }
            mini_set
        });
    let mut iter1 = (1..phi)
        .flat_map(|i|(1..phi+1)
        // .flat_map(|i|(1..=phi)
            .map(move |ii| {
                let mut mini_set = BTreeSet::new();
                insert_unique_btree!(mini_set, offset+i);
                for iii in 1..phi {
                    insert_unique_btree!(mini_set, indices_to_base_value(((ii+(iii-1)*(i) - 1)%phi)+1,iii));
                }
                mini_set
            })
        );
    let mut iter2 = (1..phi)
        .map(|i|{
            let mut mini_set = BTreeSet::new();
            for ii in 1..=phi {
                insert_unique_btree!(mini_set, indices_to_base_value(ii, i));
            }
            mini_set
        });
    
    let iters: [&mut dyn ExactSizeIterator<Item = _>; 2] = [
        &mut iter0,
        // &mut iter1,
        &mut iter2,
    ];
    // let iters: VecDeque<&mut dyn ExactSizeIterator<Item = _>> = iters.into_iter().collect::<VecDeque<_>>();
    // let iters: Vec<&mut dyn ExactSizeIterator<Item = _>> = vec![
    //     &mut iter0,
    //     // &mut iter1,
    //     &mut iter2,
    // ];
    //vec![.....].into_par_iter().flatten_iter().collect()
    vec![(0..50u8), (0..50u8)].into_par_iter()
}