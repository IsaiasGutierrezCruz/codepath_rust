// There are other conditionals that we can explore in Rust. Like using `if let`

fn main() {
    let maybe_number: Option<Option<()>> = None;
    //let maybe_number = Some(42);
    if let Some(number) = maybe_number {
        println!("The number is {:?}", number);
    } else {
        println!("There is no number");
    }

    // Case 1: Outer Option is Absent
    let maybe_number: Option<Option<()>> = None;
    if let Some(inner_option) = maybe_number {
        if let Some(_) = inner_option {
            println!("Inner value is present");
        } else {
            println!("Inner value is absent");
        }
    } else {
        println!("Outer value is absent");
    }

    // Case 2: Outer Option is Present, Inner Option is Absent
    let maybe_number: Option<Option<()>> = Some(None);
    if let Some(inner_option) = maybe_number {
        if let Some(_) = inner_option {
            println!("Inner value is present");
        } else {
            println!("Inner value is absent");
        }
    } else {
        println!("Outer value is absent");
    }

    // Case 3: Both Outer and Inner Options are Present
    let maybe_number: Option<Option<()>> = Some(Some(()));
    if let Some(inner_option) = maybe_number {
        if let Some(_) = inner_option {
            println!("Inner value is present");
        } else {
            println!("Inner value is absent");
        }
    } else {
        println!("Outer value is absent");
    }

    let maybe_number: Option<Option<i8>> = Some(None);

    if let Some(outer_option) = maybe_number {
        println!("Outer value is present {:?}", outer_option);
        if let Some(inner_option) = outer_option {
            println!("Inner value is present {:?}", inner_option);
        } else {
            println!("Inner value is absent");
        }
    } else {
        println!("Outer value is absent");
    }
}
