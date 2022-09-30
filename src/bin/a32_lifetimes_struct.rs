/*
    Topic: Lifetimes and Structures

    Requirements:
    - Display just the names and titles of persons from the mock-data.csv file
    - the names and titles must be stored in a struct separately from the mock data for potential later usage
    - none of the mock data may be duplicated in memory

    Notes:
    - the mock data has already been loaded with the include_str! macro, so all functionality must be implemented using references/borrows
*/

const MOCK_DATA: &'static str = include_str!("mock_data.csv");

struct Names<'a> {
    inner: Vec<&'a str>
}

struct Titles<'a> {
    inner: Vec<&'a str>
}

fn main() {
    let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect();
    let names: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(',').nth(1))
        .collect();

    let titles: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(',').nth(4))
        .collect();

    let names = Names {
        inner: names
    };

    let titles = Titles {
        inner: titles,
    };

    //tuple
    let combined_data = names.inner.iter().zip(titles.inner.iter());

    for (name, title) in combined_data.take(5) {
        println!("Name: {}, title: {}", name, title);
    }
       
}
