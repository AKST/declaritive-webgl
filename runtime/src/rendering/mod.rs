struct UINode {

}

trait Environment {
    fn use_memo<T>(&mut self, some_closure: impl Fn() -> T) -> T;
    fn use_callback<F, T>(&mut self, some_closure: F) -> F where F: Fn() -> T;
    fn create_element(&mut self, component: impl Component) -> UINode;
}

trait Component {
    fn render(&self, environment: &mut impl Environment) -> UINode;
}

struct UI {

}

impl Environment for UI {
    fn use_memo<T>(&mut self, some_closure: impl Fn() -> T) -> T {
        some_closure()
    }

    fn use_callback<F, T>(&mut self, some_closure: F) -> F where F: Fn() -> T {
        some_closure
    }

    fn create_element(&mut self, component: impl Component) -> UINode {
        UINode {}
    }
}

struct Message {}

impl Component for Message {
    fn render(self: &Self, stdlib: &mut impl Environment) -> UINode {
        let _memo_value = stdlib.use_memo(|| {
            2
        });

        let _on_click = stdlib.use_callback(|| {
            println!("hello world!");
        });

        UINode {}
    }
}

// fn main() {
//     let mut ui = UI {};
//     ui.create_element(Message {});
// }
