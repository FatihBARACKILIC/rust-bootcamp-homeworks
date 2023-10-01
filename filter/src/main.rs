struct FilterCondition<T> {
    filter_condition: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn new(item: T) -> Self {
        FilterCondition {
            filter_condition: item,
        }
    }

    fn is_match(&self, item: &T) -> bool {
        self.filter_condition == *item
    }
}

fn custom_filter<T: PartialEq>(item_list: Vec<T>, filter_condition: &FilterCondition<T>) -> Vec<T> {
    item_list
        .into_iter()
        .filter(|item| filter_condition.is_match(item))
        .collect()
}

fn main() {
    let item_list = vec![1, 2, 30, 4, 20, 80];
    let condition = FilterCondition::new(2);
    let filtered_list = custom_filter(item_list, &condition);
    println!("{:?}", filtered_list);
}
