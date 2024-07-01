use termedit::TermEditApp;
mod termedit;

fn main() {

    let mut app = TermEditApp::create();
    app.run().expect("Application failed to run");
}
