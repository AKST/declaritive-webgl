struct UINode {

}

trait StdLib {
    fn use_memo<T>(&mut self, some_closure: impl Fn() -> T) -> T;
    fn use_callback<F, T>(&mut self, some_closure: F) -> F where F: Fn() -> T;
    fn create_element<P>(&mut self, component: impl Fn(P, &mut Self) -> UINode, props: P) -> UINode;
}


struct UI {

}

impl StdLib for UI {
    fn use_memo<T>(&mut self, some_closure: impl Fn() -> T) -> T {
        some_closure()
    }

    fn use_callback<F, T>(&mut self, some_closure: F) -> F where F: Fn() -> T {
        some_closure
    }

    fn create_element<P>(&mut self, component: impl Fn(P, &mut Self) -> UINode, props: P) -> UINode {
        UINode {}
    }
}

impl UI {
    fn render_element<P, C>(&mut self, component: C, props: P) where C: Fn(P, &mut Self) -> UINode {
        component(props, self);
    }
}

struct MessageProps {

}

fn message_component<S: StdLib>(props: MessageProps, stdlib: &mut S) -> UINode {
    let _memo_value = stdlib.use_memo(|| {
        2
    });

    let _on_click = stdlib.use_callback(|| {
        println!("hello world!");
    });

    UINode {}
}

fn main() {
    let mut ui = UI {};
    ui.create_element(message_component, MessageProps {});
}
