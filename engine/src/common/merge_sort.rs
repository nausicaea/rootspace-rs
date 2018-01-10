/// Applies the merge-sort algorithm on a slice of partially comparable items. Its implementation
/// is from [Rosetta Code](https://rosettacode.org/wiki/Sorting_algorithms/Merge_sort#Rust).
pub fn merge_sort_with<T, F>(x: &mut [T], cmp_fn: &F) where T: Copy, F: Fn(&T, &T) -> bool {
    let mut y = x.to_vec();

    let n = x.len();
    let mut len = 1;
    while len < n {
        let mut i = 0;
        while i < n {
            if i + len >= n {
                y[i..].copy_from_slice(&x[i..]);
            } else if i + 2 * len > n {
                merge_with(&x[i..(i+len)], &x[(i+len)..], &mut y[i..], cmp_fn);
            } else {
                merge_with(&x[i..(i+len)], &x[(i+len)..(i+2*len)], &mut y[i..(i+2*len)], cmp_fn);
            }
            i += 2 * len;
        }

        len *= 2;
        if len >= n {
            x.copy_from_slice(&y);
            return;
        }

        i = 0;
        while i < n {
            if i + len >= n {
                x[i..].copy_from_slice(&y[i..]);
            } else if i + 2 * len > n {
                merge_with(&y[i..(i+len)], &y[(i+len)..], &mut x[i..], cmp_fn);
            } else {
                merge_with(&y[i..(i+len)], &y[i+len..(i+2*len)], &mut x[i..(i+2*len)], cmp_fn);
            }
            i += 2 * len;
        }

        len *= 2;
    }
}

/// Merges two slices into the output slice while sorting them.
fn merge_with<T, F>(a: &[T], b: &[T], out: &mut [T], cmp_fn: &F) where T: Copy, F: Fn(&T, &T) -> bool {
    assert_eq!(a.len() + b.len(), out.len());

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < a.len() && j < b.len() {
        if cmp_fn(&a[i], &b[j]) {
            out[k] = a[i];
            k += 1;
            i += 1;
        } else {
            out[k] = b[j];
            k += 1;
            j += 1;
        }
    }

    if i < a.len() {
        out[k..].copy_from_slice(&a[i..]);
    }
    if j < b.len() {
        out[k..].copy_from_slice(&b[j..]);
    }
}
