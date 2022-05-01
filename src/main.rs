/// This global variable refers to the title for the application.
static  APPLICATION_NAME: &str = "Datastructures and Algorithms!";

pub fn print_header(header: &str, fn_list: Vec<(&str, fn())>) {
    println!("### {}", header);
    for (sub_header, run) in fn_list {
        println!("#### {}", sub_header);
        run();
        println!("------------------\n");
    }
}

fn main() {
    println!("#############{}###############", APPLICATION_NAME);

    print_header("Multiples", vec![]);
}
