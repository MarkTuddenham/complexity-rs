fn if_fn() {
    // cc = 2
    let n = 0;
    if n > 1 {
        1;
    }
}

fn nested_if_fn() {
    // cc = 3
    let n = 0;
    if n > 1 {
        if n > 2 {
            1;
        }
    }
}

fn double_condition_if_fn() {
    // cc = 3
    let n = 0;
    let i = 1;

    if n > 1 && i < 10 {
        1;
    }

    // equivalent
    // if n > 1 {
    //     if i < 10 {
    //         1;
    //     }
    // }
}

fn double_condition_if_not_binary_fn() {
    // cc = 3
    let n = Some(0);
    let i = 1;

    if n.is_some() && i < 10 {
        1;
    }

    // equivalent
    // if n > 1 {
    //     if i < 10 {
    //         1;
    //     }
    // }
}

fn if_else_fn() {
    // cc = 2
    let n = 0;
    if n >= 1 {
        1;
    } else {
        2;
    }
}

fn if_elseif_else_fn() {
    // cc = 3
    let n = 0;
    if n > 1 {
        1;
    } else if n == 1 {
        2;
    } else {
        3;
    }

    // equivalent
    // if n > 1 {
    //     1;
    // } else {
    //     if i < 10 {
    //         2;
    //     } else {
    //         3;
    //     }
    // }
}

fn if_let_fn() {
    // cc = 2
    let some_n = Some(n);
    if let Some(n) = some_n {
        1;
    }
}

fn match_fn() {
    // cc = 2
    let some_n = Some(1);
    match some_n {
        Some(n) => 1,
        None => 2,
    };
}

fn match_tuple_fn() {
    // cc = 4
    let data = (1, 2);
    match data {
        (1, b) => 1,
        (2, b) => 2,
        (a, 1) => 3,
        (a, b) => 4,
    };
}

fn match_underscore_fn() {
    // cc = 4
    let data = (1, 2);
    match data {
        (1, _) => 1,
        (2, _) => 2,
        (_, 1) => 3,
        (_, _) => 4,
    };
}

fn match_multi_arm_fn() {
    // cc = 3
    enum Colour {
        Black,
        Blue,
        Orange,
    }

    let colour = Colour::Black;
    match colour {
        Black => 1,
        Blue => 2,
        Orange => 3,
    }
}
fn match_multi_condition_fn() {
    // cc = 3
    enum Colour {
        Black,
        Blue,
        Orange,
    }

    let colour = Colour::Black;
    match colour {
        Black => 1,
        Blue | Orange => 2,
    }
}

fn match_nonexhaustive_fn() {
    // cc = 2
    #[non_exhaustive]
    enum Test<T> {
        Some(T),
        None,
    }

    let some_n = Test::Some(1);
    match some_n {
        Some(n) => 1,
    }
}

fn if_in_match_fn() {
    // cc = 3
    let some_n = Some(n);
    match some_n {
        Some(n) => {
            if 1 == 2 {
                1
            } else {
                2
            }
        }
        None => 3,
    }
}

fn match_in_if_fn() {
    // cc = 3
    let some_n = Some(n);
    if 1 == 2 {
        match some_n {
            Some(n) => 1,
            None => 2,
        }
    }
}

fn for_fn() {
    // cc = 2
    let list = vec![1, 2, 3, 4];
    for i in list {
        i;
    }

    // equivalent
    // let mut idx = 0;
    // loop {
    //    let i = list[idx];
    //    // ...body...
    //
    //    i+=1;
    //    if i >= list.len(){
    //        break;
    //    }
    // }
}

fn while_fn() {
    // cc = 2
    let mut i = 0;
    while i < 1 {
        i += 1;
    }

    // equivalent
    // loop {
    //    if !condition {
    //        break;
    //    }
    //
    //    // ...body...
    // }
}

fn while_double_condition_fn() {
    // cc = 3
    let mut i = 0;
    while i < 3 || i % 2 {
        i += 1;
    }
}

fn iter_fn() {
    // cc = ?
    let list = vec![1, 2, 3, 4];
    list.iter().map(|i| i + 1);
}

fn iter_mut_fn() {
    // cc = ?
    let list = vec![1, 2, 3, 4];
    list.iter_mut().map(|i| i + 1);
}

fn into_iter_fn() {
    // cc = ?
    let list = vec![1, 2, 3, 4];
    list.into_iter().map(|i| i + 1);
}
