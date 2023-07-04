use std::cmp::Ordering;

pub fn print_chars(form: char, to: char) {
    let res: Ordering = form.cmp(&to);
    if res == Ordering::Less {
        for item in form..=to {
            print!("{item}");
        }
    } else if res == Ordering::Greater {
        for item in to..=form {
            print!("{item}");
        }
    } else {
        print!("{form}");
    }
}
