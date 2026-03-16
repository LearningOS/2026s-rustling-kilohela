/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord + Copy>(array: &mut [T]){

    let l = 0;
    let r = array.len() - 1;
    let mut i = l;
    let mut j = r;
    let x = array[l + ((r - l) >> 1)];

    while i <= j {
        while array[i] < x {
            i += 1;
        }
        while array[j] > x {
            j -= 1;
        }
        if i <= j {
            array.swap(i, j);
            i += 1;
            if j > 0 { j -= 1; } else {break;}
        }
    }
    if(l < j){ sort(&mut array[l..=j]);}
    if(i < r){ sort(&mut array[i..=r]);}

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}