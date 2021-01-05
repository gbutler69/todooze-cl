fn main() {
    let name = "John A";
    let min_width = 20;
    let max_width = 30;

    for idx in 1..=1000 {
        println!(
            "{Repetition:.<10} Hello, {Name:^MinWidth$.MaxWidth$}! ",
            Repetition = idx,
            Name = name,
            MinWidth = min_width,
            MaxWidth = max_width
        );
    }
}
