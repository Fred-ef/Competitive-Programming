use avl::AvlTreeMap;

pub fn eat_mosquitoes(
    frogs: Vec<(i32, i32)>,
    mosquitoes: Vec<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let mut frog_bst: AvlTreeMap<i32, (i32, usize)> = AvlTreeMap::new();
    let mut mosq_bst: AvlTreeMap<i32, i32> = AvlTreeMap::new();
    let mut result = Vec::new();

    // inizializzo il vettore soluzione come: (zanzare_mangiate, lunghezza_lingua)
    for (_, t) in frogs.iter() {
        result.push((0, *t));
    }
    

    // Inseriamo le rane (segmenti) nel BST delle rane
    for (i, (x, t)) in frogs.iter().enumerate() {
        frog_bst.insert(*x, (*t, i));
    }
    

    // Passata iniziale per rimuovere gli overlap
    let mut prev_rane: Option<(&i32, (&i32, &usize))> = None;
    let mut to_insert = Vec::new();
    let mut to_remove = Vec::new();
    for (x, (t, i)) in frog_bst.iter() {
        if *i > 0 /* && *i < rane.len() */ {
            if let Some((prev_x, (prev_t, _))) = prev_rane {
                if prev_x + prev_t >= *x {
                    // iniziamo col rimuovere l'elemento dal BST
                    to_remove.push(*x);
    
                    // se non avevamo overlap totale, dobbiamo re-inserire l'elemento
                    // rimuovendone la parte in overlap
                    if prev_x + prev_t < *x + *t {
                        to_insert.push((prev_x + prev_t + 1, (*t - (prev_x + prev_t - *x + 1), *i)));
                    }
                }
            }
        }
        prev_rane = Some((x, (t, i)));
    }
    // scriviamo le modifiche in sospeso
    for x in to_remove {
        frog_bst.remove(&x);
    }
    for (x, (t, i)) in to_insert {
        frog_bst.insert(x, (t, i));
    }

    

    // main loop (procedura online)
    for (pos, dim) in mosquitoes.iter() {
        // vettori di supporto per rimozione/aggiornamento delle rane e delle zanzare
        let mut to_insert = Vec::new();
        let mut to_remove = Vec::new();
        let mut mosq_to_remove = Vec::new();

        // Predecessor query sul BST delle rane
        if let Some((pred_x, (pred_t, pred_i))) = frog_bst.range(..=*pos).next_back() {
            // ho ottenuto il predecessore
            if pred_x+pred_t >= *pos {
                // la rana può mangiare la zanzara
                // prima parte update della rana (rimozione)
                let mut updated_len = *pred_t+*dim;
                to_remove.push(*pred_x);
                to_insert.push((*pred_x, (updated_len, *pred_i)));
                // update della rana nel vettore soluzione
                result[*pred_i].0 += 1;
                result[*pred_i].1 += *dim;
                // controllo degli overlap
                let mut curr_frog = pred_x;
                while let Some((succ_x, (succ_t, succ_i))) = frog_bst.range((curr_frog+1)..).next() {
                    if pred_x + (updated_len) >= *succ_x {
                        // c'è un overlap
                        to_remove.push(*succ_x);

                        // se non avevamo overlap totale, dobbiamo re-inserire l'elemento
                        // rimuovendone la parte in overlap
                        if pred_x + updated_len < succ_x + succ_t {
                            to_insert.push((*pred_x + updated_len + 1, (*succ_t - (*pred_x + updated_len - *succ_x + 1), *succ_i)));
                        }
                    }
                    else {
                        // nessun overlap, non iteriamo sui successori
                        break;
                    }

                    curr_frog = succ_x;
                }

                // controllo se, dopo l'update, la rana può mangiare le zanzare avanzate
                let mut mosq_index = pred_x;
                while let Some((old_mosq_pos, old_mosq_dim)) = mosq_bst.range((mosq_index+1)..).next() {
                    if pred_x+updated_len >= *old_mosq_pos {
                        // la rana può mangiare questa zanzara
                        mosq_to_remove.push(*old_mosq_pos);
                        if let Some((update_frog_x, (update_frog_t, update_frog_i))) = to_insert.pop() {
                            to_insert.push((update_frog_x, (update_frog_t + *old_mosq_dim, update_frog_i)));
                            result[update_frog_i].0 += 1;
                            result[update_frog_i].1 += *old_mosq_dim;
                            updated_len += *old_mosq_dim;
                        }
                    }
                    else {
                        // non può mangiarla, smette di iterare
                        break;
                    }

                    mosq_index = old_mosq_pos;
                }
            }
            else {
                // la rana NON può mangiare la zanzara
                // la salviamo in un BST, magari qualcuno la mangerà dopo
                mosq_bst.insert(*pos, *dim);
            }
        }

        // effettuo gli aggiornamenti delle rane in sospeso
        for x in to_remove {
            frog_bst.remove(&x);
        }
        for (x, (t, i)) in to_insert {
            frog_bst.insert(x, (t, i));
        }
        // effettuo gli aggiornamenti delle rane in sospeso
        for el in mosq_to_remove {
            mosq_bst.remove(&el);
        }
    }

    result
}