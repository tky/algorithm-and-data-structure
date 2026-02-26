fn main() {
    println!("Hello, world!");
}

fn select(vs: &mut [usize], k: usize) -> usize {
    assert!(k < vs.len());
    if vs.len() <= 5 {
        vs.sort_unstable();
        return vs[k];
    }
    let mut ms: Vec<usize> = vs.chunks_mut(5).map(median).collect();

    let len = ms.len();
    let pivot = select(ms.as_mut_slice(), len / 2);

    let (lt, gt) = partition3(vs, pivot);
    if k < lt {
        select(&mut vs[..lt], k)
    } else if k < gt {
        pivot
    } else {
        select(&mut vs[gt..], k - gt)
    }
}

fn partition3(vs: &mut [usize], pivot: usize) -> (usize, usize) {
    let mut lt = 0usize;
    let mut i = 0usize;
    let mut gt = vs.len(); // [gt..] が > pivot

    while i < gt {
        if vs[i] < pivot {
            vs.swap(i, lt);
            lt += 1;
            i += 1;
        } else if vs[i] > pivot {
            gt -= 1;
            vs.swap(i, gt);
            // i は進めない
        } else {
            i += 1;
        }
    }
    (lt, gt) // [0..lt)<, [lt..gt]==, [gt..]> 
}

fn median(vs: &mut [usize]) -> usize {
    vs.sort_unstable();
    vs[vs.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select() {
        let mut vs = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        assert_eq!(select(&mut vs, 0), 1);
        assert_eq!(select(&mut vs, 1), 1);
        assert_eq!(select(&mut vs, 2), 2);
        assert_eq!(select(&mut vs, 3), 3);
        assert_eq!(select(&mut vs, 4), 4);
        assert_eq!(select(&mut vs, 5), 5);
        assert_eq!(select(&mut vs, 6), 5);
        assert_eq!(select(&mut vs, 7), 6);
        assert_eq!(select(&mut vs, 8), 9);
    }
}
