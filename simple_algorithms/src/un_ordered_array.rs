#![allow(dead_code, unused_variables)]

pub struct UnOrderedArray {
    list: Vec<i32>,
}

impl UnOrderedArray {
    pub fn new() -> UnOrderedArray {
        UnOrderedArray { list: Vec::new() }
    }

    pub fn add_last(&mut self, item: i32) {
        self.list.push(item)
    }
    pub fn remove_last(&mut self) {
        self.list.pop();
    }

    pub fn linear_search(&self, item: i32) -> i32 {
        for i in 0..self.list.len() {
            if self.list[i] == item {
                return i as i32;
            }
        }
        return -1;
    }

    pub fn selection_sort(&mut self) {
        let end = self.list.len();
        for first_unsorted in 0..end {
            // assume firs element in array is smallest
            let mut location_of_smallest = first_unsorted;
            for i in first_unsorted..end {
                if self.list[location_of_smallest] > self.list[i] {
                    location_of_smallest = i;
                }
            }
            // once we iterated to the end of the list
            // 1 create temp variable
            let temp = self.list[first_unsorted];
            // 2 swap smallest element with first unsorted element
            self.list[first_unsorted] = self.list[location_of_smallest];
            self.list[location_of_smallest] = temp;
        }
    }

    pub fn selection_sort_desc(&mut self) {
        let end = self.list.len();
        for start in 0..end {
            let mut location_of_biggest = start;
            for i in start..end {
                if self.list[i] > self.list[location_of_biggest] {
                    location_of_biggest = i;
                }
            }

            let temp = self.list[start];
            self.list[start] = self.list[location_of_biggest];
            self.list[location_of_biggest] = temp;
        }
    }

    pub fn insertion_sort(&mut self) {
        for i in 1..self.list.len() {
            let mut j = i;
            while j > 0 && self.list[j - 1] > self.list[j] {
                self.list.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    pub fn list_items(&self) -> String {
        let mut x = String::new();

        for i in 0..self.list.len() {
            x += format!("{} ", self.list[i as usize]).as_str();
        }
        x
    }
}
