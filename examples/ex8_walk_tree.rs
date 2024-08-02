enum Tree {
    Node(i32, Vec<Tree>),
    Leaf(i32),
}

impl Tree {
    fn walk<F>(&self, f: &mut F)
    where
        F: FnMut(&i32),
    {
        match self {
            Tree::Node(val, children) => {
                f(val); // apply the func to the node
                for child in children {
                    child.walk(f);
                }
            }
            Tree::Leaf(val) => f(val), // apply the func to the leaf
        }
    }
}

fn mult_ten(x: &i32) {
    println!("multiply ten: {}", x * 10);
}

fn main() {
    let tree = Tree::Node(
        1,
        vec![
            Tree::Node(2, vec![Tree::Leaf(3), Tree::Leaf(4)]),
            Tree::Node(5, vec![Tree::Leaf(6)]),
            Tree::Leaf(7),
        ],
    );

    let mut mult_ten_func = mult_ten;
    tree.walk(&mut mult_ten_func);
}
