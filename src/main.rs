use ironhide_cli::*;

fn main() {
    let password = signin();  

    // if the sign-in fails, the app will exit
    // it will only reach here on valid sign-ins
    welcome();
    show_options();

    main_loop(&password);
}
