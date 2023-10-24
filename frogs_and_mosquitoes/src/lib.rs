use avl::AvlTreeMap;

pub fn eat_mosquitoes(
    frogs: Vec<(i32, i32)>,
    mosquitoes: Vec<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let mut frog_bst: AvlTreeMap<i32, (i32, usize)> = AvlTreeMap::new();
    let mut mosq_bst: AvlTreeMap<i32, i32> = AvlTreeMap::new();
    let mut result = Vec::new();

    // initializing the result vector as: (eaten_mosquitoes, tongue_length)
    for (_, t) in frogs.iter() {
        result.push((0, *t));
    }
    

    // inserting frog segments in the bst
    for (i, (x, t)) in frogs.iter().enumerate() {
        frog_bst.insert(*x, (*t, i));
    }
    

    // initial swipe to resolve initial overlaps
    let mut prev_rane: Option<(&i32, (&i32, &usize))> = None;
    let mut to_insert = Vec::new();
    let mut to_remove = Vec::new();
    for (x, (t, i)) in frog_bst.iter() {
        if *i > 0 {
            if let Some((prev_x, (prev_t, _))) = prev_rane {
                if prev_x + prev_t >= *x {
                    // we have to remove the element, whether to remove it or just update it
                    to_remove.push(*x);
    
                    // partial overlap, we re-insert the element without the overlapping part
                    if prev_x + prev_t < *x + *t {
                        to_insert.push((prev_x + prev_t + 1, (*t - (prev_x + prev_t - *x + 1), *i)));
                    }
                }
            }
        }
        prev_rane = Some((x, (t, i)));
    }
    // write pending modifications
    for x in to_remove {
        frog_bst.remove(&x);
    }
    for (x, (t, i)) in to_insert {
        frog_bst.insert(x, (t, i));
    }

    

    // main loop (online procedure)
    for (pos, dim) in mosquitoes.iter() {
        // support vectors for removing/updating frogs and mosquitoes
        let mut to_insert = Vec::new();
        let mut to_remove = Vec::new();
        let mut mosq_to_remove = Vec::new();

        // Predecessor query on frogs BST to find the frog on the left of the mosquitoes
        if let Some((pred_x, (pred_t, pred_i))) = frog_bst.range(..=*pos).next_back() {
            if pred_x+pred_t >= *pos {
                // the frog can eath the mosquito
                // updating the frog
                let mut updated_len = *pred_t+*dim;
                to_remove.push(*pred_x);
                to_insert.push((*pred_x, (updated_len, *pred_i)));
                // updating the frog's entry in the result vector
                result[*pred_i].0 += 1;
                result[*pred_i].1 += *dim;

                // checking overlaps
                let mut curr_frog = pred_x;
                while let Some((succ_x, (succ_t, succ_i))) = frog_bst.range((curr_frog+1)..).next() {
                    if pred_x + (updated_len) >= *succ_x {
                        // overlap found
                        to_remove.push(*succ_x);

                        // partial overlap, we re-insert the element without the overlapping part
                        if pred_x + updated_len < succ_x + succ_t {
                            to_insert.push((*pred_x + updated_len + 1, (*succ_t - (*pred_x + updated_len - *succ_x + 1), *succ_i)));
                        }
                    }
                    else {
                        // no overlap, no reason to keep iterating
                        break;
                    }

                    curr_frog = succ_x;
                }

                // check if, after the update, the frog can eat the remaining mosquitoes in the BST
                let mut mosq_index = pred_x;
                while let Some((old_mosq_pos, old_mosq_dim)) = mosq_bst.range((mosq_index+1)..).next() {
                    if pred_x+updated_len >= *old_mosq_pos {
                        // the frog can eat the mosquito: update frog and frog's entry in the result vector
                        mosq_to_remove.push(*old_mosq_pos);
                        if let Some((update_frog_x, (update_frog_t, update_frog_i))) = to_insert.pop() {
                            to_insert.push((update_frog_x, (update_frog_t + *old_mosq_dim, update_frog_i)));
                            result[update_frog_i].0 += 1;
                            result[update_frog_i].1 += *old_mosq_dim;
                            // updating the current frog's tongue length
                            updated_len += *old_mosq_dim;
                        }
                    }
                    else {
                        // the frog cannot eat the mosquito, no reason to keep iterating
                        break;
                    }

                    mosq_index = old_mosq_pos;
                }
            }
            else {
                // the frog cannot eath the mosquito, so we push it in the mosquitoes BST
                // maybe it will be eaten in the future
                mosq_bst.insert(*pos, *dim);
            }
        }

        // writing pending updates to the frogs
        for x in to_remove {
            frog_bst.remove(&x);
        }
        for (x, (t, i)) in to_insert {
            frog_bst.insert(x, (t, i));
        }
        // writing pending updates to the mosquitoes
        for el in mosq_to_remove {
            mosq_bst.remove(&el);
        }
    }

    result
}