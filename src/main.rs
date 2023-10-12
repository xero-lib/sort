fn main() {
    println!("{:?}",     basic_sort(vec![1, 2, 4, 3, 5, 4, 2]));
    println!("{:?}", insertion_sort(vec![1, 2, 4, 3, 5, 4, 2]));
    println!("{:?}",     merge_sort(vec![1, 2, 4, 3, 5, 4, 2]));
}

fn basic_sort<T>(mut input: Vec<T>) -> Vec<T>
where T: PartialOrd 
{
    for _ in 0..input.len() {
        let mut swapped = false;
        for i in 1..input.len() {
            if input[i - 1] > input[i] {
                swapped = true;
                input.swap(i, i - 1);
            }
        }
        if !swapped {
            break;
        }
    }

    /* return */ input
}

fn insertion_sort<T>(mut input: Vec<T>) -> Vec<T>
where T: PartialOrd
{
    for i in 1..input.len() {
        let mut j = i - 1;
        while j > 0 && input[j + 1] < input[j] {
            input.swap(j + 1, j);
            j -= 1;
        }
    }
    
    /* return */ input
}

fn merge_sort<T>(mut input: Vec<T>) -> Vec<T>
where T: PartialOrd
{
    todo!()
}